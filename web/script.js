/* MANDA web — theme toggle (light primary, dark secondary) + mobile nav + copy buttons */
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

  /* ----- Copy to clipboard ----- */
  function copyText(text, btn, doneLabel) {
    function success() {
      var original = btn.textContent;
      btn.textContent = doneLabel || 'Copied \u2713';
      btn.classList.add('copied');
      setTimeout(function () {
        btn.textContent = original;
        btn.classList.remove('copied');
      }, 1600);
    }
    function fallback() {
      var ta = document.createElement('textarea');
      ta.value = text;
      ta.style.position = 'fixed';
      ta.style.opacity = '0';
      document.body.appendChild(ta);
      ta.select();
      try { document.execCommand('copy'); success(); } catch (e) {}
      document.body.removeChild(ta);
    }
    if (navigator.clipboard && navigator.clipboard.writeText) {
      navigator.clipboard.writeText(text).then(success, fallback);
    } else {
      fallback();
    }
  }

  function makeCopyBtn() {
    var b = document.createElement('button');
    b.type = 'button';
    b.className = 'copy-btn';
    b.setAttribute('aria-label', 'Copy');
    b.textContent = 'Copy';
    return b;
  }

  /* Add a copy button to the hero terminal card (title bar). */
  function setupTerminalCopy() {
    var card = document.querySelector('.code-card');
    if (!card || card.querySelector('.copy-btn')) return;
    var pre = card.querySelector('pre');
    var bar = card.querySelector('.bar');
    if (!pre || !bar) return;
    var btn = makeCopyBtn();
    btn.addEventListener('click', function () {
      var text = pre.textContent
        .replace(/\u00a0/g, ' ')
        .replace(/\$ *$/, '')
        .trim();
      copyText(text + '\n', btn);
    });
    bar.appendChild(btn);
  }

  /* Add a copy button to each install command block (top-right corner). */
  function setupInstallCopy() {
    var blocks = document.querySelectorAll('.install pre');
    blocks.forEach(function (pre) {
      if (pre.parentNode.classList.contains('copy-wrap')) return;
      var wrap = document.createElement('div');
      wrap.className = 'copy-wrap';
      pre.parentNode.insertBefore(wrap, pre);
      wrap.appendChild(pre);
      var btn = makeCopyBtn();
      btn.addEventListener('click', function () {
        copyText(pre.textContent.trim() + '\n', btn);
      });
      wrap.appendChild(btn);
    });
  }

  function setup() {
    setupTerminalCopy();
    setupInstallCopy();
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', setup);
  } else {
    setup();
  }
})();
