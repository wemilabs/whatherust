const assert = require("node:assert/strict");
const { readFileSync } = require("node:fs");
const { test } = require("node:test");
const { PNG } = require("pngjs");

const icon = readFileSync(`${__dirname}/icons/icon.ico`);
const sizes = [16, 24, 32, 48, 64, 256];

function assertArtwork({ width, height, data }, minimumFill = 0.9) {
  assert.equal(width, height);
  let left = width;
  let top = height;
  let right = -1;
  let bottom = -1;
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      if (data[(y * width + x) * 4 + 3] < 128) continue;
      left = Math.min(left, x);
      top = Math.min(top, y);
      right = Math.max(right, x);
      bottom = Math.max(bottom, y);
    }
  }

  const minimum = Math.min(Math.ceil(width * minimumFill), width - 2);
  assert.ok(right - left + 1 >= minimum, `artwork is too narrow at ${width}px`);
  assert.ok(bottom - top + 1 >= minimum, `artwork is too short at ${height}px`);
  assert.ok(left > 0 && top > 0 && right < width - 1 && bottom < height - 1);
  assert.ok(Math.abs(left + right + 1 - width) <= 2, "horizontally centered");
  assert.ok(Math.abs(top + bottom + 1 - height) <= 2, "vertically centered");
}

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
    const image = PNG.sync.read(icon.subarray(offset, offset + length));
    assert.equal(image.width, size);
    assert.equal(image.height, size);
    assertArtwork(image);
  });
}

for (const [name, size] of Object.entries({
  "32x32.png": 32,
  "128x128.png": 128,
  "128x128@2x.png": 256,
  "icon.png": 512,
})) {
  test(`Linux ${name} fills its canvas without clipping`, () => {
    const image = PNG.sync.read(readFileSync(`${__dirname}/icons/${name}`));
    assert.equal(image.width, size);
    assert.equal(image.height, size);
    assertArtwork(image);
  });
}

function readIcns() {
  const buffer = readFileSync(`${__dirname}/icons/icon.icns`);
  assert.equal(buffer.toString("ascii", 0, 4), "icns");
  assert.equal(buffer.readUInt32BE(4), buffer.length);
  const chunks = new Map();
  for (let offset = 8; offset < buffer.length;) {
    const type = buffer.toString("ascii", offset, offset + 4);
    const length = buffer.readUInt32BE(offset + 4);
    assert.ok(length >= 8 && offset + length <= buffer.length);
    chunks.set(type, buffer.subarray(offset + 8, offset + length));
    offset += length;
  }
  return chunks;
}

const icns = readIcns();
for (const [type, size] of Object.entries({
  ic07: 128,
  ic08: 256,
  ic09: 512,
  ic10: 1024,
  ic11: 32,
  ic12: 64,
  ic13: 256,
  ic14: 512,
})) {
  test(`macOS ${type} (${size}px) has centered, larger Dock artwork`, () => {
    const image = PNG.sync.read(icns.get(type));
    assert.equal(image.width, size);
    assert.equal(image.height, size);
    assertArtwork(image, 0.86);
  });
}

for (const [type, size] of Object.entries({ s8mk: 16, l8mk: 32 })) {
  test(`macOS ${type} (${size}px) keeps the small-icon alpha mask aligned`, () => {
    const alpha = icns.get(type);
    assert.equal(alpha.length, size * size);
    const data = Buffer.alloc(size * size * 4);
    for (let i = 0; i < alpha.length; i++) data[i * 4 + 3] = alpha[i];
    assertArtwork({ width: size, height: size, data }, 0.86);
  });
}

test("the macOS menu-bar template is larger, centered, and monochrome", () => {
  const image = PNG.sync.read(
    readFileSync(`${__dirname}/icons/tray-template.png`),
  );
  assert.equal(image.width, 44);
  assert.equal(image.height, 44);
  assertArtwork(image, 0.86);
  for (let i = 0; i < image.data.length; i += 4) {
    if (image.data[i + 3] === 0) continue;
    assert.equal(image.data[i], 0);
    assert.equal(image.data[i + 1], 0);
    assert.equal(image.data[i + 2], 0);
  }
});
