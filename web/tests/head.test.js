#!/usr/bin/env node
/*
 * MANDA — web/head.js unit tests (language + theme boot logic).
 *
 * head.js runs synchronously in <head> of every page (EN and ES) before the
 * first paint. These tests pin its behavior so a regression in the
 * language auto-detect, the EN<->ES redirect, the back/forward guard or
 * the page structure fails CI on every push.
 *
 * Zero dependencies: only node:assert, node:fs, node:path, node:vm.
 * Run:  node web/tests/head.test.js   (exit code 0 = green, 1 = red)
 */
'use strict';

const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const vm = require('node:vm');

const REPO_ROOT = path.resolve(__dirname, '../..');
const HEAD_JS = path.join(REPO_ROOT, 'web', 'head.js');
const code = fs.readFileSync(HEAD_JS, 'utf8');

let passed = 0;
let failed = 0;

/* Execute head.js in a stubbed browser sandbox and report what it did. */
function runHead(opts) {
  const {
    pathname,
    search = '',
    hash = '',
    savedLang = null, /* null = no localStorage entry for manda-lang */
    savedTheme = null,
    browserLang = 'en',
    navType = 'navigate',
  } = opts;
  let replaced = null;
  const stored = {};
  const attrs = {};
  const sandbox = {
    localStorage: {
      getItem(k) {
        if (k === 'manda-lang' && savedLang !== null) return savedLang;
        if (k === 'manda-theme' && savedTheme !== null) return savedTheme;
        return stored[k] !== undefined ? stored[k] : null;
      },
      setItem(k, v) { stored[k] = String(v); },
    },
    navigator: { language: browserLang, languages: [browserLang] },
    location: {
      pathname,
      search,
      hash,
      replace: (u) => { replaced = u; },
    },
    document: {
      documentElement: {
        setAttribute: (k, v) => { attrs[k] = v; },
      },
    },
    performance: {
      getEntriesByType: () => [{ type: navType }],
      navigation: null,
    },
    console,
  };
  vm.createContext(sandbox);
  vm.runInContext(code, sandbox);
  return { replaced, stored, attrs };
}

function test(name, fn) {
  try {
    fn();
    passed += 1;
    console.log(`  \u2713 ${name}`);
  } catch (err) {
    failed += 1;
    console.error(`  \u2717 ${name}\n    ${String(err.message).split('\n')[0]}`);
  }
}

/* ------------------------------------------------------------------ */
/* Language: persist + redirect only on mismatch + back/forward guard  */
/* ------------------------------------------------------------------ */

console.log('=== Language: first visit (auto-detect persists) ===');

test('es browser on /index.html redirects to es/index.html', () => {
  const r = runHead({ pathname: '/index.html', browserLang: 'es-ES' });
  assert.equal(r.replaced, 'es/index.html');
});

test('first es visit persists manda-lang=es', () => {
  const r = runHead({ pathname: '/index.html', browserLang: 'es-ES' });
  assert.equal(r.stored['manda-lang'], 'es');
});

test('es browser already on /es/index.html does NOT redirect and persists', () => {
  const r = runHead({ pathname: '/es/index.html', browserLang: 'es' });
  assert.equal(r.replaced, null);
  assert.equal(r.stored['manda-lang'], 'es');
});

test('en browser on /index.html stays put and persists manda-lang=en', () => {
  const r = runHead({ pathname: '/index.html', browserLang: 'en-US' });
  assert.equal(r.replaced, null);
  assert.equal(r.stored['manda-lang'], 'en');
});

console.log('=== Language: mismatch redirects preserve hash and query ===');

test('saved es + /index.html#download -> es/index.html#download', () => {
  const r = runHead({ pathname: '/index.html', hash: '#download', savedLang: 'es' });
  assert.equal(r.replaced, 'es/index.html#download');
});

test('saved es + /features.html#ai -> es/features.html#ai', () => {
  const r = runHead({ pathname: '/features.html', hash: '#ai', savedLang: 'es' });
  assert.equal(r.replaced, 'es/features.html#ai');
});

test('query string survives the redirect (?utm_source=gh)', () => {
  const r = runHead({
    pathname: '/index.html', search: '?utm_source=gh', hash: '#download', savedLang: 'es',
  });
  assert.equal(r.replaced, 'es/index.html?utm_source=gh#download');
});

test('saved en + /es/faq.html -> ../faq.html', () => {
  const r = runHead({ pathname: '/es/faq.html', savedLang: 'en' });
  assert.equal(r.replaced, '../faq.html');
});

test('saved en + /es/faq.html?x=1 -> ../faq.html?x=1', () => {
  const r = runHead({ pathname: '/es/faq.html', search: '?x=1', savedLang: 'en' });
  assert.equal(r.replaced, '../faq.html?x=1');
});

test('root path / with es browser -> es/index.html', () => {
  const r = runHead({ pathname: '/', browserLang: 'es' });
  assert.equal(r.replaced, 'es/index.html');
});

console.log('=== Language: saved choice beats browser language ===');

test('saved en + es browser stays in EN (no redirect)', () => {
  const r = runHead({ pathname: '/index.html', savedLang: 'en', browserLang: 'es' });
  assert.equal(r.replaced, null);
});

test('saved es + en browser redirects to ES', () => {
  const r = runHead({ pathname: '/index.html', savedLang: 'es', browserLang: 'en' });
  assert.equal(r.replaced, 'es/index.html');
});

console.log('=== Language: back/forward never bounces ===');

test('back/forward to /es/features.html with saved en does NOT redirect', () => {
  const r = runHead({
    pathname: '/es/features.html', hash: '#ai', savedLang: 'en', browserLang: 'es', navType: 'back_forward',
  });
  assert.equal(r.replaced, null);
});

test('back/forward to /index.html with saved es does NOT redirect', () => {
  const r = runHead({
    pathname: '/index.html', savedLang: 'es', browserLang: 'en', navType: 'back_forward',
  });
  assert.equal(r.replaced, null);
});

console.log('=== Language: /es without trailing slash edge ===');

test('/es (no slash) + es -> treated as ES root, no redirect', () => {
  const r = runHead({ pathname: '/es', savedLang: 'es', browserLang: 'en' });
  assert.equal(r.replaced, null);
});

test('/es (no slash) + en -> ../index.html (no self-loop)', () => {
  const r = runHead({ pathname: '/es', savedLang: 'en', browserLang: 'es' });
  assert.equal(r.replaced, '../index.html');
});

/* ------------------------------------------------------------------ */
/* Theme                                                                */
/* ------------------------------------------------------------------ */

console.log('=== Theme ===');

test('persisted dark theme applies data-theme="dark" before paint', () => {
  const r = runHead({ pathname: '/index.html', savedTheme: 'dark' });
  assert.equal(r.attrs['data-theme'], 'dark');
});

test('no saved theme leaves data-theme unset (light default)', () => {
  const r = runHead({ pathname: '/index.html' });
  assert.equal(r.attrs['data-theme'], undefined);
});

/* ------------------------------------------------------------------ */
/* Structure: every page must load head.js, zero inline boot scripts    */
/* ------------------------------------------------------------------ */

function collectPages() {
  const pages = [];
  (function walk(dir, prefix) {
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      if (entry.isDirectory()) {
        walk(path.join(dir, entry.name), prefix + entry.name + '/');
      } else if (entry.name.endsWith('.html')) {
        pages.push({ rel: prefix + entry.name, abs: path.join(dir, entry.name) });
      }
    }
  })(path.join(REPO_ROOT, 'web'), '');
  return pages;
}

console.log('=== Structure: 8 pages reference head.js, zero inline boot scripts ===');

test('at least 8 HTML pages exist (4 EN + 4 ES)', () => {
  assert.ok(collectPages().length >= 8);
});

test('every page references head.js with the correct relative path', () => {
  for (const { rel, abs } of collectPages()) {
    const html = fs.readFileSync(abs, 'utf8');
    const isEs = rel.startsWith('es/');
    const needle = isEs ? 'src="../head.js"' : 'src="head.js"';
    assert.ok(html.includes(needle), `${rel} must reference ${needle}`);
  }
});

test('no page contains inline boot scripts (theme/lang live only in head.js)', () => {
  for (const { rel, abs } of collectPages()) {
    const html = fs.readFileSync(abs, 'utf8');
    const inlineBoot = /<script>(?:(?!<\/script>)[\s\S])*manda-(?:theme|lang)/.test(html);
    assert.ok(!inlineBoot, `${rel} must not contain inline theme/lang scripts`);
  }
});

test('head.js stays dependency-free (no require/import)', () => {
  assert.ok(!/\brequire\s*\(|\bimport\s+/.test(code), 'head.js must not require/import anything');
});

/* ------------------------------------------------------------------ */

console.log(`\n${passed} passed, ${failed} failed`);
process.exitCode = failed ? 1 : 0;
