const assert = require("node:assert/strict");
const { readFileSync } = require("node:fs");
const { test } = require("node:test");
const { runInNewContext } = require("node:vm");

const script = readFileSync(`${__dirname}/src/notification-gate.js`, "utf8");

function setup(enabled = true) {
  const requests = [];
  const delivered = [];
  let permission = "default";
  const Original = function () {
    assert.fail("the ungated plugin constructor must not be called");
  };
  Object.defineProperty(Original, "permission", {
    get: () => permission,
    set: (value) => {
      permission = value;
    },
  });
  Original.requestPermission = async () => {
    window.Notification.permission = "granted";
    return "granted";
  };
  const window = {
    Notification: Original,
    __TAURI_INTERNALS__: {
      invoke(command, { options }) {
        assert.equal(command, "show_notification");
        requests.push(options);
        if (enabled) delivered.push(options);
        return Promise.resolve();
      },
    },
  };
  runInNewContext(script, { window });
  return {
    window,
    Original,
    requests,
    delivered,
    setEnabled(value) {
      enabled = value;
    },
  };
}

test("native delivery always uses the gated command, preserving desktop options and id", () => {
  const state = setup();
  const options = {
    id: 42,
    body: "Message",
    icon: "icon.png",
    sound: "default",
    title: "Wrong",
  };
  const notification = new state.window.Notification("Alice", options);
  assert.equal(state.delivered.length, 1);
  assert.equal(state.delivered[0].title, "Alice");
  assert.equal(state.delivered[0].id, 42);
  assert.equal(state.delivered[0].body, "Message");
  assert.equal(state.delivered[0].icon, "icon.png");
  assert.equal(state.delivered[0].sound, "default");
  assert.equal(options.title, "Wrong");
  assert.ok(notification instanceof state.Original);
  assert.ok(notification instanceof state.window.Notification);
});

test("live settings and reloads never cache an enabled value in JavaScript", () => {
  const state = setup();
  new state.window.Notification("Enabled");
  state.setEnabled(false);
  new state.window.Notification("Disabled");
  assert.equal(state.requests.length, 2);
  assert.equal(state.delivered.length, 1);
  const reloaded = setup(false);
  new reloaded.window.Notification("Still disabled after reload");
  assert.equal(reloaded.delivered.length, 0);
  state.setEnabled(true);
  new state.window.Notification("Re-enabled");
  assert.equal(state.delivered.length, 2);
});

test("permission initialization and permission requests still use the plugin accessors", async () => {
  const state = setup();
  state.window.Notification.permission = "default";
  assert.equal(state.window.Notification.permission, "default");
  assert.equal(await state.window.Notification.requestPermission(), "granted");
  assert.equal(state.Original.permission, "granted");
});

test("delivery failures do not fall back to the ungated plugin constructor", async () => {
  const state = setup();
  state.window.__TAURI_INTERNALS__.invoke = () =>
    Promise.reject(new Error("IPC unavailable"));
  new state.window.Notification("Unavailable");
  await new Promise((resolve) => setImmediate(resolve));
  assert.equal(state.delivered.length, 0);
});

test("remote capability cannot bypass the native gate or change settings", () => {
  const remote = JSON.parse(
    readFileSync(`${__dirname}/capabilities/whatsapp-web.json`, "utf8"),
  );
  assert.deepEqual(remote.permissions, [
    "allow-show-notification",
    "notification:allow-is-permission-granted",
    "notification:allow-request-permission",
  ]);
  const local = JSON.parse(
    readFileSync(`${__dirname}/capabilities/settings.json`, "utf8"),
  );
  assert.deepEqual(local.windows, ["settings"]);
  assert.equal(local.remote, undefined);
});
