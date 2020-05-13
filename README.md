# rust-ray-intersect-js

Ray and mesh intersect implementation with Rust for JavaScript. Contains implementation for bounding spheres and
triangles.

## How to Run BabylonJS Performance Test

1) Install Rust (https://www.rust-lang.org/tools/install)
2) Install Wasm-Pack  (https://rustwasm.github.io/wasm-pack/installer/)
3) Run speedtest_wasm_vs_babylon.sh (Note: May take a while to get all of the rust items you need)

## Project Structure

This project follows rust wasm_pack project template structure (wasm-pack-template).

Additionally, 'ts'-folder contains typescript integration tests for node (Mocha) and browsers (Karma).
Webpack folder contains also example of webpack configuration for loading the WASM with base64 loader.

## Build and Test

When interface does not change it is enough to run the following commands. 

NOTE: If JavaScript interface changes then pkg/intersect.d.ts and pkg/web/intersect.js need
to be changed by hand. This is due to the fact that wasm_bindgen does not automatically generate
webpack friendly (bundler target requires wasm_bindgen specific webpack loader) nor isomorphic npm packages.

```
./build.sh
./test.sh
```

## Release

Public patch release to NPM:

```
./publish.sh
```

Publish other release type to NPM:

```
cd pkg
npm version major | minor | patch
npm publish
```

Initial scoped public publish to NPM:

```
cd pkg
npm publish --access=public
```

Deprecate a version from NPM:

```
cd pkg
npm deprecate @tlaukkan/intersect@"< 0.2.3" "critical bug fixed in v0.2.3"
```

Remove a version from NPM:

```
cd pkg
npm unpublish @tlaukkan/intersect@0.1.0
```

## Initial project setup notes

Template generated on macOS Catalina.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cargo install cargo-generate --features vendored-openssl
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name rust-ray-intersect-js
```
    
