use rhai::{packages::Package, Engine, EvalAltResult};
use rhai_http::{HttpPackage,TestStruct};


#[test]
fn test_http() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_global_module(HttpPackage::new().as_shared_module());
    engine.register_type_with_name::<TestStruct>("TestStruct")
        .register_fn("new_ts", TestStruct::new);

    let variable_result = engine.eval::<i64>(r#"
let ST = new_ts();

ST.field = 2;
ST.field
"#)?;

    assert!(2 == variable_result);

    Ok(())
}
