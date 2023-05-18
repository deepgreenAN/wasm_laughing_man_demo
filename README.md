# 笑い男デモ

wasmの顔検出を用いた笑い男デモ．顔検出は[rustface](https://github.com/atomashpolskiy/rustface)を用いています．UIにsvelteを用いていいます．rust(leptos)を用いたバージョンは[こちら](https://github.com/deepgreenAN/wasm_laughing_man_demo_v2/).

## requirements

- wasm-pack

## build

```shell
cd wasm
wasm-pack build --target web --release
cd ..
npm install
npm run build
npm run serve
```
