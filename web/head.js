/*
 * MANDA — head boot script (theme + language).
 *
 * Loaded SYNCHRONOUSLY in <head> on every page (EN and ES), BEFORE first
 * paint, to prevent FOUC:
 *   - applies the persisted dark theme before the page renders
 *   - auto-redirects first-time visitors whose browser is Spanish to /es/
 *     (EN stays primary), while respecting a manual EN/ES override in
 *     localStorage ("manda-lang").
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

  /* ---------- Language auto-detect + manual override ---------- */
  try {
    var saved = localStorage.getItem('manda-lang');
    var lang = saved;
    if (!lang) {
      var nav = (navigator.language || (navigator.languages && navigator.languages[0]) || 'en')
        .toLowerCase();
      lang = nav.indexOf('es') === 0 ? 'es' : 'en';
    }
    var onEs = /\/es\//.test(location.pathname);
    if (lang === 'es' && !onEs) {
      var page = location.pathname.split('/').pop() || 'index.html';
      location.replace('es/' + page);
    } else if (lang === 'en' && onEs) {
      location.replace('../' + (location.pathname.split('/').pop() || 'index.html'));
    }
  } catch (e) {}
})();
