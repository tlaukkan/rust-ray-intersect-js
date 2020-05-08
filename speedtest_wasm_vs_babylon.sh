#!/usr/bin/env bash
wasm-pack build --scope tlaukkan --target nodejs
wasm-pack test --headless --firefox

cd ts/node && \
npm install && \
npm test && \
cd ../..
