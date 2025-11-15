# Wasmtime host component hello world

This repo functions as a proof of concept for implementing a simple wasmtime host component.

The wit directory contains the contract: a simple `capitalize` function to turn a string into its uppercase variant.

```wit
package my:capitalize;

interface host-functions {
    capitalize: func(input: string) -> string;
}

world app {
    import host-functions;
    export run: func(input: string) -> string;
}
```

The code in `guest/` implements the wasm component that will use the capitalize function implemented by the host in `host/`.

to build the guest component:

```bash
cd guest
cargo build --target wasm32-wasip2
```

To run the host component:

```bash
cd host
cargo run
```
