# Third-party notices

This crate redistributes a vendored copy of the upstream
[`@ninoverse/hmi-components`](https://github.com/ninoverse/hmi-components) bundle,
committed under `assets/vendor/` and injected at runtime by `HmiAssets`:

- `hmi-components.iife.js` — the custom-element registration script
- `hmi-components.css` — component styles
- `hmi-constants.css`, `hmi-color-default.css`, `hmi-structure-default.css` — theme tokens

These files are taken verbatim from `@ninoverse/hmi-components` version `5.0.1`
(see `xtask/`, which re-vendors them). They are distributed under the MIT License,
reproduced below. `hmi-dioxus` itself is also MIT-licensed (see `LICENSE`); the two
licenses are compatible.

---

## @ninoverse/hmi-components

MIT License

Copyright (c) 2026 Nicola

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
