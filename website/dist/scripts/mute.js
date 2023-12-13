!(function(e) {
  const t = {};
  function r(n) {
    if (t[n]) return t[n].exports;
    const o = (t[n] = { i: n, l: !1, exports: {} });
    return e[n].call(o.exports, o, o.exports, r), (o.l = !0), o.exports;
  }
  (r.m = e),
    (r.c = t),
    (r.d = function(e, t, n) {
      r.o(e, t) || Object.defineProperty(e, t, { enumerable: !0, get: n });
    }),
    (r.r = function(e) {
      `undefined` !== typeof Symbol &&
        Symbol.toStringTag &&
        Object.defineProperty(e, Symbol.toStringTag, { value: `Module` }),
        Object.defineProperty(e, `__esModule`, { value: !0 });
    }),
    (r.t = function(e, t) {
      if ((1 & t && (e = r(e)), 8 & t)) return e;
      if (4 & t && `object` === typeof e && e && e.__esModule) return e;
      const n = Object.create(null);
      if (
        (r.r(n),
        Object.defineProperty(n, `default`, { enumerable: !0, value: e }),
        2 & t && `string` !== typeof e)
      )
        for (const o in e)
          r.d(
            n,
            o,
            function(t) {
              return e[t];
            }.bind(null, o)
          );
      return n;
    }),
    (r.n = function(e) {
      const t =
        e && e.__esModule
          ? function() {
              return e.default;
            }
          : function() {
              return e;
            };
      return r.d(t, `a`, t), t;
    }),
    (r.o = function(e, t) {
      return Object.prototype.hasOwnProperty.call(e, t);
    }),
    (r.p = ``),
    r((r.s = 0));
})([function(e, t) {}]);
!(function() {
  if (!window.requestAnimationFrame) {
    let a = null;
    window.requestAnimationFrame = function(t, i) {
      const e = new Date().getTime();
      a || (a = e);
      const s = Math.max(0, 16 - (e - a));
      const n = window.setTimeout(function() {
        t(e + s);
      }, s);
      return (a = e + s), n;
    };
  }
  function r(t, i, e, s) {
    return (t /= s / 2) < 1
      ? (e / 2) * t * t * t * t + i
      : (-e / 2) * ((t -= 2) * t * t * t - 2) + i;
  }
  function e(t) {
    return t ? [1, 0] : [0, 1];
  }
  function t(t) {
    (this.element = t),
      (this.svg = (function t(i) {
        let e = i.parentNode;
        if (`svg` !== e.tagName) e = t(e);
        return e;
      })(this.element)),
      this.getSize(),
      (this.states = this.element.querySelectorAll(
        `.js-transition-icon__state`
      )),
      (this.time = { start: null, total: 200 }),
      (this.status = { interacted: !1, animating: !1 }),
      (this.animation = {
        effect: this.element.getAttribute(`data-effect`),
        event: this.element.getAttribute(`data-event`),
      }),
      this.init();
  }
  (t.prototype.getSize = function() {
    const t = this.svg.getAttribute(`viewBox`);
    this.size = t
      ? { width: t.split(` `)[2], height: t.split(` `)[3] }
      : this.svg.getBoundingClientRect();
  }),
    (t.prototype.init = function() {
      const t = this;
      this.svg.addEventListener(t.animation.event, function() {
        t.status.animating ||
          ((t.status.animating = !0),
          window.requestAnimationFrame(t.triggerAnimation.bind(t)));
      });
    }),
    (t.prototype.triggerAnimation = function(t) {
      const i = this.getProgress(t);
      this.animateIcon(i), this.checkProgress(i);
    }),
    (t.prototype.getProgress = function(t) {
      return this.time.start || (this.time.start = t), t - this.time.start;
    }),
    (t.prototype.checkProgress = function(t) {
      if (this.time.total > t)
        window.requestAnimationFrame(this.triggerAnimation.bind(this));
      else {
        (this.status = {
          interacted: !this.status.interacted,
          animating: !1,
        }),
          (this.time.start = null);
        const i = e(this.status.interacted);
        this.states[i[0]].removeAttribute(`aria-hidden`),
          this.states[i[1]].setAttribute(`aria-hidden`, `true`);
      }
    }),
    (t.prototype.animateIcon = function(t) {
      if (t > this.time.total) {
        t = this.time.total;
      }
      if (t < 0) {
        t = 0;
      }
      const i = e(this.status.interacted);
      (this.states[i[0]].style.display =
        t > this.time.total / 2 ? `none` : `block`),
        (this.states[i[1]].style.display =
          t > this.time.total / 2 ? `block` : `none`),
        `scale` == this.animation.effect
          ? this.scaleIcon(t, i[0], i[1])
          : this.rotateIcon(t, i[0], i[1]);
    }),
    (t.prototype.scaleIcon = function(t, i, e) {
      const s = r(
        Math.min(t, this.time.total / 2),
        1,
        -0.2,
        this.time.total / 2
      ).toFixed(2);
      const n = r(
        Math.max(t - this.time.total / 2, 0),
        0.2,
        -0.2,
        this.time.total / 2
      ).toFixed(2);
      this.states[i].setAttribute(
        `transform`,
        `translate(${(this.size.width * (1 - s)) / 2} ${(this.size.height *
          (1 - s)) /
          2}) scale(${s})`
      ),
        this.states[e].setAttribute(
          `transform`,
          `translate(${(this.size.width * n) / 2} ${(this.size.height * n) /
            2}) scale(${1 - n})`
        );
    }),
    (t.prototype.rotateIcon = function(t, i, e) {
      const s = r(t, 0, 180, this.time.total).toFixed(2);
      this.states[i].setAttribute(
        `transform`,
        `rotate(-${s} ${this.size.width / 2} ${this.size.height / 2})`
      ),
        this.states[e].setAttribute(
          `transform`,
          `rotate(${180 - s} ${this.size.width / 2} ${this.size.height / 2})`
        );
    });
  const i = document.querySelectorAll(`.js-transition-icon`);
  if (i) for (let s = 0; i.length > s; s++) new t(i[s]);
})();
