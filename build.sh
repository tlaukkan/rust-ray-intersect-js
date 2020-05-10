#!/usr/bin/env bash
wasm-pack build --release --scope tlaukkan --target nodejs --out-dir pkg2/node
cp pkg2/node/intersect_bg.wasm pkg/node/intersect_bg.wasm
cp pkg2/node/intersect.js pkg/node/intersect.js

wasm-pack build --release --scope tlaukkan --target web --out-dir pkg2/web
cp pkg2/web/intersect_bg.wasm pkg/bundler/intersect_bg.wasm