# rhai-http

Rhai Http is a plugin extension for doing http requests inside Rhai lang

## Running examples

```
cargo run --example run_code
```

## Usage

### `Cargo.toml`

```toml
[dependencies]
rhai-http = "0.1"
```

### Add the module plugin into the Rhai Engine

```
let mut engine = Engine::new();

engine.register_global_module(HttpPackage::new().as_shared_module());
engine.register_type_with_name::<Http>("Http")
    .register_fn("new_http", Http::new);
engine.register_type_with_name::<Http>("HttpResponse");
```

### [Rhai] script

```js
let variable = function_name("ENV_VARIABLE")

function_name("ENV_VARIABLE", variable)
```