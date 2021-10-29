# 笑い男デモ
wasmの顔検出を用いた笑い男デモ
## requirements
- wasm-pack

## build
```
cd wasm
wasm-pack build --target web --release
cd ..
npm install
npm run build
npm run serve
```
