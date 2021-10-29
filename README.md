# 笑い男デモ
wasmの顔検出を用いた笑い男デモ
顔検出は[rustface](https://github.com/atomashpolskiy/rustface)を用いている．
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
