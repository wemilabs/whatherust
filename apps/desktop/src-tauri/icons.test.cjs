const assert = require("node:assert/strict");
const { readFileSync } = require("node:fs");
const { test } = require("node:test");
const { PNG } = require("pngjs");

const icon = readFileSync(`${__dirname}/icons/icon.ico`);
const sizes = [16, 24, 32, 48, 64, 256];

test("the Windows bundle uses an ICO with native small and large frames", () => {
  const config = JSON.parse(
    readFileSync(`${__dirname}/tauri.conf.json`, "utf8"),
  );
  assert.ok(config.bundle.icon.includes("icons/icon.ico"));
  assert.equal(icon.readUInt16LE(0), 0);
  assert.equal(icon.readUInt16LE(2), 1);
  assert.equal(icon.readUInt16LE(4), sizes.length);
});

for (const [index, size] of sizes.entries()) {
  test(`the ${size}px Windows icon fills its canvas without clipping`, () => {
    const entry = 6 + index * 16;
    assert.equal(icon[entry] || 256, size);
    assert.equal(icon[entry + 1] || 256, size);
    const length = icon.readUInt32LE(entry + 8);
    const offset = icon.readUInt32LE(entry + 12);
    const { width, height, data } = PNG.sync.read(
      icon.subarray(offset, offset + length),
    );
    assert.equal(width, size);
    assert.equal(height, size);

    let left = size;
    let top = size;
    let right = -1;
    let bottom = -1;
    for (let y = 0; y < size; y++) {
      for (let x = 0; x < size; x++) {
        if (data[(y * size + x) * 4 + 3] < 128) continue;
        left = Math.min(left, x);
        top = Math.min(top, y);
        right = Math.max(right, x);
        bottom = Math.max(bottom, y);
      }
    }

    const minimum = size === 16 ? 14 : Math.ceil(size * 0.9);
    assert.ok(
      right - left + 1 >= minimum,
      `artwork is too narrow at ${size}px`,
    );
    assert.ok(bottom - top + 1 >= minimum, `artwork is too short at ${size}px`);
    assert.ok(left > 0 && top > 0 && right < size - 1 && bottom < size - 1);
    assert.ok(Math.abs(left + right + 1 - size) <= 2, "horizontally centered");
    assert.ok(Math.abs(top + bottom + 1 - size) <= 2, "vertically centered");
  });
}
