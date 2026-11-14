const assert = require("node:assert/strict");
const { readFileSync } = require("node:fs");
const { test } = require("node:test");
const { runInNewContext } = require("node:vm");

const script = readFileSync(`${__dirname}/src/notification-click.js`, "utf8");

function setup(titles, { missingPane = false, repeatLabel = false } = {}) {
  const rows = titles.map(() => ({
    events: [],
    dispatchEvent(event) { this.events.push(event.type); },
  }));
  const labels = titles.map((title, index) => ({
    getAttribute: () => title,
    closest: () => rows[index],
  }));
  if (repeatLabel) labels.push(labels[0]);
  let id;
  let focused = 0;
  const window = {
    Notification: function (_title, options) { id = options.id; },
    focus: () => focused++,
  };
  runInNewContext(script, {
    window,
    document: { getElementById: () => missingPane ? null : { querySelectorAll: () => labels } },
    Event,
    EventTarget,
    MouseEvent: Event,
    setTimeout: () => {},
  });
  return {
    rows,
    create: (title = "Chat") => new window.Notification(title),
    nativeClick: () => window.__whatherustNotificationClick(id),
    click(title, options) {
      new window.Notification(title, options);
      window.__whatherustNotificationClick(id);
      assert.equal(focused, 1);
    },
  };
}

test("permission init and requests assign through the wrapper without bypassing protection", async () => {
  const window = {};
  const context = {
    window, Event, EventTarget, setTimeout: () => {},
    invoke: async (command) => command === "is_permission_granted" ? true : "granted",
  };
  // Mirrors tauri-plugin-notification 2.7.0's src/init.js permission closure.
  runInNewContext(`
    "use strict";
    let permissionWritable = false;
    let permission = "default";
    function setPermission(value) {
      permissionWritable = true;
      window.Notification.permission = value;
      permissionWritable = false;
    }
    window.Notification = function () {};
    window.Notification.requestPermission = async function () {
      return await invoke("request_permission").then((value) => {
        setPermission(value === "prompt" || value === "prompt-with-rationale" ? "default" : value);
        return value;
      });
    };
    Object.defineProperty(window.Notification, "permission", {
      enumerable: true,
      get: () => permission,
      set: (value) => {
        if (!permissionWritable) throw new Error("Readonly property");
        permission = value;
      },
    });
    window.init = (async function () {
      return window.Notification.permission !== "default"
        ? await Promise.resolve(window.Notification.permission === "granted")
        : await invoke("is_permission_granted");
    })().then((value) => setPermission(value === null ? "default" : value ? "granted" : "denied"));
  `, context);
  runInNewContext(script, context);
  await window.init;
  assert.equal(window.Notification.permission, "granted");
  assert.equal(await window.Notification.requestPermission(), "granted");
  assert.throws(() => { window.Notification.permission = "denied"; }, /Readonly property/);
  assert.equal(window.Notification.permission, "granted");
});

test("once listeners stop suppressing fallback after dispatch", () => {
  const state = setup(["Chat"]);
  const notification = state.create();
  let calls = 0;
  const listener = () => calls++;
  notification.addEventListener("click", listener, { once: true });
  notification.addEventListener("click", listener, false);
  notification.dispatchEvent(new Event("click"));
  state.nativeClick();
  assert.equal(calls, 1);
  assert.deepEqual(state.rows[0].events, ["mousedown", "mouseup", "click"]);
});

test("active once listener handles native click without fallback", () => {
  const state = setup(["Chat"]);
  const notification = state.create();
  let calls = 0;
  notification.addEventListener("click", { handleEvent() { calls++; } }, { once: true });
  state.nativeClick();
  notification.dispatchEvent(new Event("click"));
  assert.equal(calls, 1);
  assert.deepEqual(state.rows[0].events, []);
});

for (const preAborted of [false, true]) {
  test(`aborted signal stops suppressing fallback (pre-aborted: ${preAborted})`, () => {
    const state = setup(["Chat"]);
    const notification = state.create();
    const controller = new AbortController();
    let calls = 0;
    if (preAborted) controller.abort();
    notification.addEventListener("click", () => calls++, { signal: controller.signal });
    controller.abort();
    state.nativeClick();
    assert.equal(calls, 0);
    assert.deepEqual(state.rows[0].events, ["mousedown", "mouseup", "click"]);
  });
}

test("removal matches original listener and capture independently", () => {
  const state = setup(["Chat"]);
  const notification = state.create();
  let calls = 0;
  const listener = () => calls++;
  notification.addEventListener("click", listener);
  notification.addEventListener("click", listener, { capture: true });
  notification.removeEventListener("click", listener);
  notification.dispatchEvent(new Event("click"));
  assert.equal(calls, 1);
  notification.removeEventListener("click", listener, { capture: true });
  state.nativeClick();
  assert.equal(calls, 1);
  assert.deepEqual(state.rows[0].events, ["mousedown", "mouseup", "click"]);
});

test("unique title opens only its matching row", () => {
  const state = setup(["Other", "Chat"]);
  state.click("Chat");
  assert.deepEqual(state.rows.map((row) => row.events), [[], ["mousedown", "mouseup", "click"]]);
});

test("duplicate titles stay focus-only even with notification metadata", () => {
  const state = setup(["Chat", "Chat"]);
  state.click("Chat", { body: "Hello", tag: "chat-id", data: { chatId: "chat-id" } });
  assert.ok(state.rows.every((row) => row.events.length === 0));
});

test("multiple matching labels in the same row are not ambiguous", () => {
  const state = setup(["Chat"], { repeatLabel: true });
  state.click("Chat");
  assert.deepEqual(state.rows[0].events, ["mousedown", "mouseup", "click"]);
});

for (const [name, titles, title, options] of [
  ["empty list", [], "Chat", {}],
  ["unmatched title", ["Other"], "Chat", {}],
  ["empty title", [""], "", {}],
  ["missing pane", ["Chat"], "Chat", { missingPane: true }],
]) {
  test(`${name} stays focus-only`, () => {
    const state = setup(titles, options);
    state.click(title);
    assert.ok(state.rows.every((row) => row.events.length === 0));
  });
}
