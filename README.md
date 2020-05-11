### For Babylonjs Forum test
```

1) Install Rust (https://www.rust-lang.org/tools/install)
2) Install Wasm-Pack  (https://rustwasm.github.io/wasm-pack/installer/)
3) Run speedtest_wasm_vs_babylon.sh (Note: May take a while to get all of the rust items you need)

```
# rust-ray-intersect-js

Sandbox project for ray intersect implementation with Rust for JavaScript.

## Build and Test

When interface does not change run the following. If interface changes then pkg/intersect.d.ts and pkg/web/intersect.js need
to be changed by hand.

```
./build.sh
./test.sh
```

## Publish

### Initial scoped public publish to NPM

```
cd pkg
npm publish --access=public
```

### Publish

```
cd pkg
npm publish
```

### Deprecate from NPM

```
cd pkg
npm deprecate @tlaukkan/intersect@"< 0.2.3" "critical bug fixed in v0.2.3"
```

### Unpublish a version from NPM

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
    
