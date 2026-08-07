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

  /* ----- Language override: persist the visitor's manual choice ----- */
  function setupLangOverride() {
    var links = document.querySelectorAll('.lang a');
    if (!links.length) return;
    links.forEach(function (a) {
      a.addEventListener('click', function () {
        var lang = (a.textContent || '').trim().toLowerCase() === 'es' ? 'es' : 'en';
        try { localStorage.setItem('manda-lang', lang); } catch (e) {}
      });
    });
  }

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

  /* ----- Hero terminal typing effect + cyclic shortcut carousel -----
     Types the initial command once, then loops forever through all the
     keyboard shortcuts, one per cycle, so the demo keeps living. */
  function setupTyping() {
    var card = document.querySelector('.code-card');
    var cmd = card && card.querySelector('.cmd-type');
    if (!card || !cmd) return;
    var typed = cmd.querySelector('.typed');
    var cmdCursor = cmd.querySelector('.cursor');
    var doneLine = card.querySelector('.done-line');
    var promptFinal = card.querySelector('.prompt-final');
    var cycleText = card.querySelector('.cycle-text');
    var cycleResult = card.querySelector('.cycle-result');
    if (!typed || !cycleText) return;

    /* Pause the carousel while hovered or focused (WCAG 2.2.2). */
    var paused = false;
    card.addEventListener('mouseenter', function () { paused = true; });
    card.addEventListener('mouseleave', function () { paused = false; });
    card.addEventListener('focusin', function () { paused = true; });
    card.addEventListener('focusout', function () { paused = false; });

    var reduced = window.matchMedia && window.matchMedia('(prefers-reduced-motion: reduce)').matches;

    /* One entry per shortcut shown in the carousel: the key combo and its label. */
    var es = (document.documentElement.getAttribute('lang') || 'en').toLowerCase().indexOf('es') === 0;
    var shortcuts = es ? [
      { cmd: 'Cmd+Shift+A', out: 'Panel de IA' },
      { cmd: 'Cmd+T', out: 'Nueva pesta\u00f1a' },
      { cmd: 'Cmd+L', out: 'Chat de IA' },
      { cmd: 'Cmd+Shift+G', out: 'Abrir lazygit' },
      { cmd: 'Cmd+D', out: 'Dividir panel en vertical' },
      { cmd: 'Cmd+K', out: 'Limpiar pantalla' }
    ] : [
      { cmd: 'Cmd+Shift+A', out: 'AI Panel' },
      { cmd: 'Cmd+T', out: 'New Tab' },
      { cmd: 'Cmd+L', out: 'AI Chat' },
      { cmd: 'Cmd+Shift+G', out: 'Open Lazygit' },
      { cmd: 'Cmd+D', out: 'Split Pane Vertical' },
      { cmd: 'Cmd+K', out: 'Clear Screen' }
    ];

    if (reduced) {
      typed.textContent = cmd.getAttribute('data-cmd') || '';
      if (doneLine) doneLine.classList.add('visible');
      if (promptFinal) promptFinal.classList.add('visible');
      if (cycleResult) {
        cycleResult.querySelector('.cycle-out').textContent = shortcuts[0].out;
        cycleResult.classList.add('visible');
      }
      return;
    }

    var speed = 55; /* ms per char — natural typing pace */

    function typeText(el, text, done) {
      var n = 0;
      el.textContent = '';
      (function step() {
        n += 1;
        el.textContent = text.slice(0, n);
        if (n < text.length) {
          setTimeout(step, speed + Math.random() * 25);
        } else if (done) {
          done();
        }
      })();
    }

    var idx = 0;

    function runCycle() {
      if (paused) { setTimeout(runCycle, 400); return; }
      var item = shortcuts[idx % shortcuts.length];
      idx += 1;
      if (cycleResult) cycleResult.classList.remove('visible');
      cycleText.classList.remove('hiding');
      typeText(cycleText, item.cmd, function () {
        setTimeout(function () {
          if (cycleResult) {
            cycleResult.querySelector('.cycle-out').textContent = item.out;
            cycleResult.classList.add('visible');
          }
          setTimeout(function () {
            cycleText.classList.add('hiding');
            setTimeout(function () {
              cycleText.textContent = '';
              cycleText.classList.remove('hiding');
              if (cycleResult) cycleResult.classList.remove('visible');
              runCycle();
            }, 220);
          }, 1700);
        }, 350);
      });
    }

    /* First type the deploy command, reveal the ✓ line, then start the carousel. */
    typeText(typed, cmd.getAttribute('data-cmd') || '', function () {
      if (cmdCursor) cmdCursor.style.opacity = '0';
      setTimeout(function () {
        if (doneLine) doneLine.classList.add('visible');
        setTimeout(function () {
          if (promptFinal) promptFinal.classList.add('visible');
          setTimeout(runCycle, 600);
        }, 450);
      }, 350);
    });
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
    setupLangOverride();
    setupInstallCopy();
    setupTyping();
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', setup);
  } else {
    setup();
  }
})();
