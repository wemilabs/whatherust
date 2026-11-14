(() => {
  const Polyfill = window.Notification;
  if (typeof Polyfill !== "function") return;
  window.Notification = new Proxy(Polyfill, {
    construct(_target, [title, options], newTarget) {
      // The native command checks settings; direct plugin delivery is not permitted.
      window.__TAURI_INTERNALS__
        .invoke("show_notification", { options: { ...options, title } })
        .catch(() => {});
      return Object.create(newTarget.prototype);
    },
  });
})();
