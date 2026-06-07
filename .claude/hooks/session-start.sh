#!/bin/bash
# SessionStart hook for Claude Code on the web.
#
# Sets up the Rust + Dioxus toolchain so `cargo`, `dx`, the linter, the test
# suite, and `dx build`/`dx serve` (web) all work in a fresh remote container.
#
# Why this is needed: the remote environment routes egress through a
# TLS-intercepting proxy. `dx` downloads its helper tools (esbuild, binaryen,
# wasm-bindgen) with rustls + bundled roots, which rejects the proxy cert, so
# those downloads fail. `curl`/`npm` use the system CA (which trusts the proxy),
# so we fetch the helper binaries here and drop them into dx's tool cache.
#
# Versions are pinned to the Dioxus CLI version below. When you bump the
# `dioxus` crate / `dx`, update DX_VERSION and the tool versions to match
# (run `dx build --platform web` once and read the versions dx asks for).
set -euo pipefail

# Only run in Claude Code on the web; local machines manage their own toolchain.
if [ "${CLAUDE_CODE_REMOTE:-}" != "true" ]; then
  exit 0
fi

# --- pinned versions (must match the installed dx) ---------------------------
DX_VERSION="0.7.9"
ESBUILD_VERSION="0.27.3"
WASM_BINDGEN_VERSION="0.2.122"
BINARYEN_DIR_VERSION="129"   # cache dir dx looks in: binaryen-<this>
BINARYEN_DL_VERSION="127"    # release dx actually downloads: version_<this>

ARCH="x86_64"
CARGO_BIN="${CARGO_HOME:-$HOME/.cargo}/bin"
TOOLS_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/.dx/tools"
mkdir -p "$CARGO_BIN" "$TOOLS_DIR"
export PATH="$CARGO_BIN:$PATH"

# --- 1. wasm target (needed to build/clippy the web platform) ----------------
rustup target add wasm32-unknown-unknown

# --- 2. Dioxus CLI -----------------------------------------------------------
# `cargo install dioxus-cli` fails here (git2/libssh2 not available), so grab
# the prebuilt binary from the GitHub release instead.
if ! command -v dx >/dev/null 2>&1; then
  tmp="$(mktemp -d)"
  curl -fsSL -o "$tmp/dx.tar.gz" \
    "https://github.com/DioxusLabs/dioxus/releases/download/v${DX_VERSION}/dx-${ARCH}-unknown-linux-gnu.tar.gz"
  tar -xzf "$tmp/dx.tar.gz" -C "$tmp"
  install -m755 "$tmp/dx" "$CARGO_BIN/dx"
  rm -rf "$tmp"
fi

# --- 3. esbuild (JS/CSS bundling) -------------------------------------------
esbuild_dir="$TOOLS_DIR/esbuild-${ESBUILD_VERSION}"
if [ ! -x "$esbuild_dir/esbuild" ]; then
  tmp="$(mktemp -d)"
  curl -fsSL -o "$tmp/esbuild.tgz" \
    "https://registry.npmjs.org/@esbuild/linux-x64/-/linux-x64-${ESBUILD_VERSION}.tgz"
  tar -xzf "$tmp/esbuild.tgz" -C "$tmp"
  mkdir -p "$esbuild_dir"
  install -m755 "$tmp/package/bin/esbuild" "$esbuild_dir/esbuild"
  rm -rf "$tmp"
fi

# --- 4. binaryen / wasm-opt (release wasm optimization) ---------------------
binaryen_dir="$TOOLS_DIR/binaryen-${BINARYEN_DIR_VERSION}"
if [ ! -x "$binaryen_dir/bin/wasm-opt" ]; then
  tmp="$(mktemp -d)"
  curl -fsSL -o "$tmp/binaryen.tar.gz" \
    "https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_DL_VERSION}/binaryen-version_${BINARYEN_DL_VERSION}-${ARCH}-linux.tar.gz"
  tar -xzf "$tmp/binaryen.tar.gz" -C "$tmp"
  mkdir -p "$binaryen_dir"
  cp -r "$tmp/binaryen-version_${BINARYEN_DL_VERSION}/bin" "$binaryen_dir/"
  cp -r "$tmp/binaryen-version_${BINARYEN_DL_VERSION}/lib" "$binaryen_dir/"
  rm -rf "$tmp"
fi

# --- 5. wasm-bindgen-cli (wasm <-> JS glue) ---------------------------------
wb_dir="$TOOLS_DIR/wasm-bindgen-${WASM_BINDGEN_VERSION}"
if [ ! -x "$wb_dir/wasm-bindgen" ]; then
  tmp="$(mktemp -d)"
  curl -fsSL -o "$tmp/wb.tar.gz" \
    "https://github.com/rustwasm/wasm-bindgen/releases/download/${WASM_BINDGEN_VERSION}/wasm-bindgen-${WASM_BINDGEN_VERSION}-${ARCH}-unknown-linux-musl.tar.gz"
  tar -xzf "$tmp/wb.tar.gz" -C "$tmp"
  mkdir -p "$wb_dir"
  install -m755 "$tmp/wasm-bindgen-${WASM_BINDGEN_VERSION}-${ARCH}-unknown-linux-musl/wasm-bindgen" "$wb_dir/wasm-bindgen"
  rm -rf "$tmp"
fi

# --- 6. cache crate dependencies --------------------------------------------
cargo fetch --manifest-path "${CLAUDE_PROJECT_DIR:-.}/Cargo.toml"

echo "Rust + Dioxus toolchain ready (dx ${DX_VERSION})."
