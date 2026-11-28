// Run: node tests/gui/settings-registered.mjs
// Every select in the Settings modal is registered with the store, so it is kept between launches and reverts.
import assert from 'node:assert';
import fs from 'node:fs';

const html = fs.readFileSync(new URL('../../src/gui/index.html', import.meta.url), 'utf8');
const store = fs.readFileSync(new URL('../../src/gui/js/settings-store.js', import.meta.url), 'utf8');

const missing = [...html.matchAll(/<select id="([^"]+)"/g)]
  .map((m) => m[1])
  .filter((id) => !store.includes(`id: '${id}'`));
assert.deepStrictEqual(missing, [], 'a select the store does not know resets every launch');
console.log('settings-registered: ok');
