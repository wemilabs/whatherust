(() => {
  const Polyfill = window.Notification;
  if (typeof Polyfill !== "function") return;
  // Each document starts muted until its current setting arrives from Rust.
  let enabled = false;
  const sync = () =>
    window.__TAURI_INTERNALS__
      .invoke("notifications_enabled")
      .then((value) => { enabled = value === true; })
      .catch(() => { enabled = false; });
  Object.defineProperty(window, "__whatherustSyncNotifications", { value: sync });
  sync();
  window.Notification = new Proxy(Polyfill, {
    construct(target, args, newTarget) {
      return enabled
        ? Reflect.construct(target, args, newTarget)
        : Object.create(target.prototype);
    },
  });
})();
