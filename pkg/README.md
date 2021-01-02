# Rust & WebAssembly Markdown Convertor

Rust Markdown convertor which complies to wasm(WebAssembly). So it can be used in both back-end and front-end.

## Rust compile to js(wasm)

Will use [Wasm-pack](https://rustwasm.github.io/wasm-pack/) to complie that.

```
wasm-pack build --target web --out-name index
```

### Use wasm js in the web

After complie you can use this wasm library in your code

1. Step 1
   import libiray

```
import init, { convert } from '../pkg/index.js';
```

2. Step 2
   Init, wasm-bindgen will load the files

```
await init();
```

3. Step 3
   call `convert` function

```
const markdown = `
            # H1 Title
        `;
const result = convert(markdown);
```
