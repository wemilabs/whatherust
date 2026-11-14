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
    click(title, options) {
      new window.Notification(title, options);
      window.__whatherustNotificationClick(id);
      assert.equal(focused, 1);
    },
  };
}

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
