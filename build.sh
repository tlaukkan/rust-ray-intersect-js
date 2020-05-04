#!/usr/bin/env bash
wasm-pack build --release --scope tlaukkan --target nodejs --out-dir pkg/node && cp pkg/node/intersect.d.ts pkg/intersect.d.ts