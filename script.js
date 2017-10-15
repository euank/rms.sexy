// This is free and unencumbered software released into the public domain.
//
// Anyone is free to copy, modify, publish, use, compile, sell, or distribute
// this software, either in source code form or as a compiled binary, for any
// purpose, commercial or non-commercial, and by any means.
//
// The source code, along with the full license text, is available here:
// https://github.com/Mechazawa/rms.sexy

(function() {
	"use strict";

	function StallmanImage(url) {
		this.url = url;
		this.cached = null;

		this.show = function(el, state) {
			if (this.cached !== null) {
				return this.display(el, state);
			}

			let xhr = new XMLHttpRequest();
			let self = this;
			xhr.addEventListener('readystatechange', function() {
				if (this.readyState == 4) {
					let r = new FileReader();
					r.addEventListener('load', function() {
						self.cached = this.result;
						self.display(el, state);
					});
					r.readAsDataURL(xhr.response);
				}
			});
			xhr.responseType = 'blob';
			xhr.open('GET', this.url);
			xhr.send();
		};

		this.display = function(el, state) {
			el.src = this.cached;
			if (state) {
				console.log('pushing state');
				history.pushState({img: this.url}, '', '#' + this.url);
			}
		};
	}

	function StallmanImageReloader() {
		this.img = null;
		this.reloadTime = 5000;
		this.memoryEater = new JavaSimulator();
		this.images = [];
		this.imagesByUrl = {};
		this.interval = null;

		this.feed = function(imgs) {
			for (let img of imgs) {
				this.memoryEater.feed();
				let io = new StallmanImage(img);
				this.images.push(io);
				this.imagesByUrl[img] = io;
			}
		};

		this.setImageElement = function(img) {
			this.img = img;
			let self = this;
			img.onclick = function(ev) {
				self.reloadImage();
				self.run();

				ev.stopPropagation();
				return false;
			};
			img.style.cursor = 'pointer';
		};

		this.setImage = function(url) {
			if (this.imagesByUrl.hasOwnProperty(url)) {
				this.imagesByUrl[url].show(this.img, false);
				this.run();
			}
		};

		this.run = function() {
			if (this.interval !== null) {
				clearInterval(this.interval);
			}
			let self = this;
			this.interval = setInterval(function() { self.reloadImage(); }, self.reloadTime);
		};

		this.reloadImage = function() {
			let img = this.images[Math.floor(Math.random() * this.images.length)];
			this.memoryEater.feed();
			img.show(this.img, true);
		};
	}

	function JavaSimulator() {
		this.garbage = {};
		this.totalGarbage = 0;

		// TODO: extend this into an actual managed cache for images
		this.feed = function() {
			let o = Math.random();
			let l = [];
			for (let i=0; i<o*16; ++i) {
				l.push(i * o);
			}
			this.garbage[o] = l;
			this.totalGarbage += o;
			if (this.totalGarbage > 1024*1024/8) {
				this.gc();
			}
		};

		this.gc = function() {
			if (Math.random() > 0.1) {
				return;
			}
			let threshold = Math.random();
			for (var k in this.garbage) {
				if (k < threshold) {
					if (!this.garbage[k].pop()) {
						delete this.garbage[k];
						this.totalGarbage -= k;
					}
				}
			}
		};
	}

	function StallmanImageReloaderFactory() {
		this.singleton = null;

		this._manufacture = function() {
			return new StallmanImageReloader();
		}

		this.getStallmanImageReloaderSingletonInstance = function() {
			if (this.singleton === null) {
				this.singleton = this._manufacture();
			}
			return this.singleton;
		};
	}
	StallmanImageReloaderFactory.stallmanImageReloaderFactorySingleton = new StallmanImageReloaderFactory();

	function hasThing(thing) {
		let o = window;
		for (let part of thing.split('.')) {
			if (!o) {
				return false;
			}
			o = o[part];
		}
		return !!o;
	}

	function checkRequired(things) {
		let ok = true;
		for (let thing of things) {
			if (!hasThing(thing)) {
				console.log('required browser feature missing: ' + thing);
				ok = false;
			}
		}
		return ok;
	}

	function getImgs() {
		let xhr = new XMLHttpRequest();
		xhr.addEventListener('readystatechange', function() {
			if (this.readyState == 4) {
				let imgs = [];
				for (let img of JSON.parse(this.responseText)) {
					imgs.push(img);
				}
				let stallman = document.querySelector('.stallman');
				let p = stallman.parentElement;
				p.insertAdjacentElement('beforebegin', stallman);
				p.remove();
				StallmanImageReloaderFactory.stallmanImageReloaderFactorySingleton.getStallmanImageReloaderSingletonInstance().feed(imgs);
				StallmanImageReloaderFactory.stallmanImageReloaderFactorySingleton.getStallmanImageReloaderSingletonInstance().setImageElement(stallman);
				StallmanImageReloaderFactory.stallmanImageReloaderFactorySingleton.getStallmanImageReloaderSingletonInstance().run();
				window.onhashchange = function() {
					StallmanImageReloaderFactory.stallmanImageReloaderFactorySingleton.getStallmanImageReloaderSingletonInstance().setImage(window.location.hash.substring(1));
				};
				if (window.location.hash.length > 1) {
					window.onhashchange();
				} else {
					StallmanImageReloaderFactory.stallmanImageReloaderFactorySingleton.getStallmanImageReloaderSingletonInstance().reloadImage();
				}
			}
		});
		xhr.open('GET', '?images');
		xhr.send();
	}

	function hasRefresh() {
		for (let meta of document.getElementsByTagName('meta')) {
			if (meta.httpEquiv == "refresh") {
				return true;
			}
		}
		return false;
	}

	function init() {
		if (!checkRequired(['history.pushState', 'XMLHttpRequest', 'JSON.parse', 'FileReader'])) {
			return;
		}

		if (hasRefresh()) {
			window.location = '?js';
		}

		getImgs()
	}

	if (document.readyState !== 'loading') {
		init()
	} else {
		document.addEventListener('DOMContentLoaded', init);
	}
})();
