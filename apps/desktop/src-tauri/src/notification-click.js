// Runs after the notification plugin's `window.Notification` polyfill. The polyfill returns a
// bare object, so WhatsApp's `onclick` never fires; this keeps each page notification and, when
// the native one is clicked, the `on_action` handler in lib.rs calls the hook below with its id.
(() => {
  const Polyfill = window.Notification;
  if (typeof Polyfill !== "function" || window.__whatherustNotificationClick) return;

  const MAX_TRACKED = 100;
  const shown = new Map();
  const internals = new WeakMap();
  let nextId = Math.floor(Math.random() * 0x3fffffff);

  // Listeners are keyed by (listener, capture), matching how EventTarget dedupes and removes them.
  const isCapture = (options) => Boolean(typeof options === "boolean" ? options : options && options.capture);

  const fire = (notification, type) => {
    const event = new Event(type, { cancelable: type === "click" });
    notification.dispatchEvent(event);
    // EventTarget subclasses don't run `on<type>` properties by themselves.
    const handler = notification[`on${type}`];
    if (typeof handler === "function") handler.call(notification, event);
    return event;
  };

  // Fallback when the page attached no click handler: open the chat list row titled like the notification.
  const openChatTitled = (title) => {
    const pane = document.getElementById("pane-side");
    if (!pane || !title) return;
    const label = Array.from(pane.querySelectorAll("span[title]")).find(
      (span) => span.getAttribute("title") === title,
    );
    const row = label && (label.closest('[role="listitem"], [role="row"], [role="gridcell"]') || label);
    if (!row) return;
    for (const type of ["mousedown", "mouseup", "click"]) {
      row.dispatchEvent(new MouseEvent(type, { bubbles: true, cancelable: true, view: window }));
    }
  };

  class Notification extends EventTarget {
    constructor(title, options) {
      super();
      const settings = options || {};
      nextId = (nextId % 0x7ffffffe) + 1;
      const id = nextId;
      internals.set(this, { id, closed: false, clickListeners: [new Set(), new Set()] });

      this.title = String(title);
      this.body = settings.body === undefined ? "" : String(settings.body);
      this.tag = settings.tag === undefined ? "" : String(settings.tag);
      this.icon = settings.icon === undefined ? "" : String(settings.icon);
      this.data = settings.data === undefined ? null : settings.data;
      this.silent = settings.silent === undefined ? null : Boolean(settings.silent);
      this.requireInteraction = Boolean(settings.requireInteraction);
      this.onclick = null;
      this.onshow = null;
      this.onclose = null;
      this.onerror = null;

      shown.set(id, this);
      if (shown.size > MAX_TRACKED) shown.delete(shown.keys().next().value);
      // A copy, since the polyfill freezes what it sends; `id` comes back with the native click.
      new Polyfill(this.title, { ...settings, id });
      setTimeout(() => fire(this, "show"));
    }

    addEventListener(type, listener, options) {
      if (type === "click" && listener) internals.get(this).clickListeners[+isCapture(options)].add(listener);
      super.addEventListener(type, listener, options);
    }

    removeEventListener(type, listener, options) {
      if (type === "click") internals.get(this).clickListeners[+isCapture(options)].delete(listener);
      super.removeEventListener(type, listener, options);
    }

    // The plugin can't dismiss desktop toasts, so stay tracked: a click on the lingering toast
    // should still open the chat.
    close() {
      const state = internals.get(this);
      if (state.closed) return;
      state.closed = true;
      fire(this, "close");
    }

    static get permission() {
      return Polyfill.permission;
    }

    static requestPermission(callback) {
      return Polyfill.requestPermission().then((state) => {
        const permission =
          state === "prompt" || state === "prompt-with-rationale" ? "default" : state;
        if (typeof callback === "function") callback(permission);
        return permission;
      });
    }

    static get maxActions() {
      return 0;
    }
  }

  Object.defineProperty(window, "__whatherustNotificationClick", {
    value: (id) => {
      const notification = shown.get(id);
      if (!notification) return;
      shown.delete(id);
      window.focus();
      const handled =
        typeof notification.onclick === "function" || internals.get(notification).clickListeners.some((listeners) => listeners.size > 0);
      fire(notification, "click");
      if (!handled) openChatTitled(notification.title);
      notification.close();
    },
  });

  window.Notification = Notification;
})();
