const assert = require("node:assert/strict");
const { readFileSync } = require("node:fs");
const { test } = require("node:test");
const { runInNewContext } = require("node:vm");

const script = readFileSync(`${__dirname}/src/notification-gate.js`, "utf8");

function setup() {
  let resolve;
  let reject;
  let delivered = 0;
  const window = {
    Notification: function () { delivered++; },
    __TAURI_INTERNALS__: {
      invoke(command) {
        assert.equal(command, "notifications_enabled");
        return new Promise((yes, no) => { resolve = yes; reject = no; });
      },
    },
  };
  window.Notification.permission = "granted";
  runInNewContext(script, { window });
  return {
    window,
    get delivered() { return delivered; },
    async update(enabled) {
      resolve(enabled);
      await Promise.resolve();
    },
    async fail() {
      reject(new Error("IPC unavailable"));
      await Promise.resolve();
      await Promise.resolve();
    },
  };
}

test("every page load stays muted until Rust confirms notifications are enabled", async () => {
  const state = setup();
  new state.window.Notification("Before settings");
  assert.equal(state.delivered, 0);
  await state.update(true);
  new state.window.Notification("Enabled");
  assert.equal(state.delivered, 1);
  const reloaded = setup();
  new reloaded.window.Notification("Reloaded before settings");
  await reloaded.update(false);
  new reloaded.window.Notification("Disabled");
  assert.equal(reloaded.delivered, 0);
});

test("the page can only re-read the Rust setting and cannot enable notifications", async () => {
  const state = setup();
  await state.update(false);
  const syncing = state.window.__whatherustSyncNotifications(true);
  await state.update(false);
  await syncing;
  new state.window.Notification("Still disabled");
  assert.equal(state.delivered, 0);
});

test("live changes use the new value and IPC failure mutes delivery", async () => {
  const state = setup();
  await state.update(true);
  new state.window.Notification("Enabled");
  const disabling = state.window.__whatherustSyncNotifications();
  await state.update(false);
  await disabling;
  new state.window.Notification("Disabled");
  const failing = state.window.__whatherustSyncNotifications();
  await state.fail();
  await failing;
  new state.window.Notification("Unavailable");
  assert.equal(state.delivered, 1);
  assert.equal(state.window.Notification.permission, "granted");
});
