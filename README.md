# Running Rust in Deno JS 

## Creating wasm from Rust: 

- Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Install [Rust](https://www.rust-lang.org/tools/install)

From project root type: 

```bash
cd my-wasm
wasm-pack build 
```

The target WASM code is created inside `my-wasm/pkg`

## Calling WASM in Deno 

From project root type: 

```bash
 deno run --allow-read app.js
```

Output: 

```bash
result from WASM =  100
```
