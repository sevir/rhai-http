use rhai::{packages::Package, Engine, EvalAltResult, ImmutableString};
use rhai_http::*;


#[test]
fn test_http_struct() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_global_module(HttpPackage::new().as_shared_module());
    engine.register_type_with_name::<Http>("Http")
        .register_fn("new_http", Http::new);

    let variable_result = engine.eval::<ImmutableString>(r#"
let http = new_http();

http.cookie
"#)?;

    assert!("" == variable_result.as_str());

    Ok(())
}

#[test]
fn test_http_get() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_global_module(HttpPackage::new().as_shared_module());
    engine.register_type_with_name::<Http>("Http")
        .register_fn("new_http", Http::new);
    engine.register_type_with_name::<Http>("HttpResponse");

    let variable_result = engine.eval::<i64>(r#"
let http = new_http();

let response = http.get("https://www.digio.es");
response.code
"#)?;

    assert!(200 == variable_result);

    Ok(())
}