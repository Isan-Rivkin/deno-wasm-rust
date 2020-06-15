// import wasm 
const wasmCode = await Deno.readFile("./my-wasm/pkg/my_wasm_bg.wasm")
const wasmModule = new WebAssembly.Module(wasmCode)
const wasmInstance = new WebAssembly.Instance(wasmModule)
const myWasm = wasmInstance.exports

const result = myWasm.compute(10)
console.log("result from WASM = " , result )


