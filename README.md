# lshoo space

Everything about lshoo: education, learning, thinking ...

Build with [Spin](https://github.com/fermyon/spin)

## Usage
Below is an example of using the `spin` CLI to create a new Spin application.  To run the example you will need to install the `wasm32-wasi` target for Rust.

```bash
$ rustup target add wasm32-wasi
```

 Build the application with `spin build`, then run it locally with `spin up`:

```bash
# Compile to Wasm by executing the `build` command.
$ spin build
Executing the build command for component hello-rust: cargo build --target wasm32-wasi --release
    Finished release [optimized] target(s) in 0.03s
Successfully ran the build command for the Spin components.

# Run the application locally.
# `--key-value` options in `spin up` require version 1.2.0
$ spin up --key-value DB_URL="{pg-url-yourself}
Logging component stdio to ".spin/logs/"

Serving http://127.0.0.1:3000
Available Routes:
  spaces: http://127.0.0.1:3000 (wildcard)
```



```bash
# Deploy to Spin Cloud
$ spin deploy --key-value DB_URL="{pg-url-yourself}

Deploying...
Waiting for application to become ready...... ready
Available Routes:
  spaces: https://lshoo.fermyon.app (wildcard)
```    