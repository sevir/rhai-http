# rhai-http

Rhai Http is a plugin extension for doing http requests inside Rhai lang

Add blocking request get and post, with headers support, json encoding and decoding

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
let mut engine = Engine::new(); // Rhai parser

engine.register_global_module(HttpPackage::new().as_shared_module());
engine.register_type_with_name::<Http>("Http")
    .register_fn("new_http", Http::new);
```

### [Rhai] script

```js
let http = new_http();

let response = http.get("http://httpbin.org/get");
print(response.code);

let http_client = new_http();
http_client.set_header("mitest", "mitestvalue");
let body = #{
    "hola":"mundo"
};

let body_json = serialize(body);
http_client.set_body(body_json);
let response = http_client.post("http://httpbin.org/post");
print(response.body);
let response_map = parse(response.body);
print(response_map.origin)
```