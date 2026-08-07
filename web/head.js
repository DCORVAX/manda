/*
 * MANDA — head boot script (theme + language).
 *
 * Loaded SYNCHRONOUSLY in <head> on every page (EN and ES), BEFORE first
 * paint, to prevent FOUC:
 *   - applies the persisted dark theme before the page renders
 *   - resolves the visitor's language ONCE and persists it, so navigating
 *     between pages keeps the same language version without re-detecting
 *     and re-redirecting (no redirect flash on every page load); a manual
 *     EN/ES click in the nav overrides the stored choice via script.js
 *   - redirects to /es/ only when the resolved language differs from the
 *     current page's language, preserving URL hashes (e.g. #download)
 *   - never redirects on back/forward navigation, so going back to a page
 *     you already viewed never bounces you to the other language
 *
 * Keep this file dependency-free and defensive: no DOM queries, no network,
 * wrapped in try/catch so a storage exception can never break the page.
 */
(function () {
  'use strict';

  /* ---------- Theme (must run before first paint) ---------- */
  try {
    if (localStorage.getItem('manda-theme') === 'dark') {
      document.documentElement.setAttribute('data-theme', 'dark');
    }
  } catch (e) {}

  /* ---------- Language: resolve once, persist, redirect only on mismatch ---------- */
  try {
    /* Detect whether this page was reached via back/forward (bfcache or
       history traversal). In that case the visitor chose to return to a
       page they already saw — never bounce them to the other language. */
    var navType = 'other';
    try {
      var navEntries = performance.getEntriesByType && performance.getEntriesByType('navigation');
      if (navEntries && navEntries.length) {
        navType = navEntries[0].type || 'other';
      } else if (performance.navigation) {
        navType = performance.navigation.type === 2 ? 'back_forward' : 'other';
      }
    } catch (e) {}

    var saved = localStorage.getItem('manda-lang');
    var lang = saved;
    if (!lang) {
      var nav = (navigator.language || (navigator.languages && navigator.languages[0]) || 'en')
        .toLowerCase();
      lang = nav.indexOf('es') === 0 ? 'es' : 'en';
      /* Persist the detected language so the whole browsing session keeps
         the same version without detect + redirect flash on each page. A
         manual EN/ES click later overwrites it via script.js. */
      try { localStorage.setItem('manda-lang', lang); } catch (e2) {}
    }

    if (navType === 'back_forward') return;

    var page = location.pathname.split('/').pop() || 'index.html';
    /* '/es' without a trailing slash is the Spanish root on both hosts, but
       the regex below needs the slash — treat an exact 'es' segment as ES. */
    var onEs = /\/es\//.test(location.pathname) || page === 'es';
    /* Preserve query strings (e.g. ?utm_source=...) and hashes on redirect. */
    var suffix = location.search + location.hash;
    if (lang === 'es' && !onEs) {
      location.replace('es/' + page + suffix);
    } else if (lang === 'en' && onEs) {
      /* '/es' without slash must not bounce to itself ('../es') — go to the
         EN home instead. */
      var target = page === 'es' ? 'index.html' : page;
      location.replace('../' + target + suffix);
    }
  } catch (e) {}
})();
