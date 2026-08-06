/* MANDA web — theme toggle (light primary, dark secondary) + mobile nav */
(function () {
  'use strict';

  function currentTheme() {
    return document.documentElement.getAttribute('data-theme') === 'dark' ? 'dark' : 'light';
  }

  window.toggleTheme = function () {
    var next = currentTheme() === 'dark' ? 'light' : 'dark';
    document.documentElement.setAttribute('data-theme', next);
    try { localStorage.setItem('manda-theme', next); } catch (e) {}
  };

  window.toggleNav = function () {
    var links = document.querySelector('.nav-links');
    if (!links) return;
    var open = links.classList.toggle('open');
    var btn = document.querySelector('.nav-toggle');
    if (btn) {
      btn.setAttribute('aria-expanded', open ? 'true' : 'false');
      btn.setAttribute('aria-label', open ? 'Close menu' : 'Menu');
    }
  };
})();
