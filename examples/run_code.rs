extern crate rhai-http;

use rhai::{Engine, EvalAltResult};
use rhai::packages::Package; 
use rhai_http::*;

fn main() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new(); // Rhai parser
    engine.register_global_module(HttpPackage::new().as_shared_module());
    engine.register_type_with_name::<Http>("Http")
        .register_fn("new_http", Http::new);

    engine.run_file("examples/code.rhai".into())?;

    Ok(())
}