# rust-ray-intersect-js

Sandbox project for ray intersect implementation with Rust for JavaScript.

### Build with `wasm-pack build`

```
wasm-pack build --scope tlaukkan
```

### Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### Initial scoped public publish to NPM

```
npm publish --access=public
```

### Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

### Deprecate from NPM

```
npm deprecate @tlaukkan/intersect@"< 0.2.3" "critical bug fixed in v0.2.3"
```

### Unpublish a version from NPM

```
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
    
 