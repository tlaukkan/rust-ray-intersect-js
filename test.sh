#!/usr/bin/env bash
cargo test && \
cd ts/webpack && \
npm test && \
cd ../.. && \
cd ts/node && \
npm test && \
cd ../..
