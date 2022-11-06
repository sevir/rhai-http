use rhai::def_package;
use rhai::plugin::*;

#[derive(Debug, Clone)]
pub struct TestStruct {
    field: i64
}

impl TestStruct {
    pub fn new() -> Self {
        Self { field: 1 }
    }

    fn set_field(&mut self, new_val: i64){
        self.field = new_val;
    }

    fn get_field(&mut self) -> i64 {
        self.field.clone()
    }
}

def_package! {
    /// Package for read and write env variables
    pub HttpPackage(lib) {
        combine_with_exported_module!(lib, "http", http_functions);
    }    
}

#[export_module]
mod http_functions{
    #[rhai_fn(name = "new_struct")]
    pub fn new_struct() -> TestStruct {
        TestStruct::new()
    }

    #[rhai_fn(get = "field", pure)]
    pub fn get_field(element: &mut TestStruct) -> i64 {
        element.get_field()
    }

    #[rhai_fn(set = "field")]
    pub fn set_field(element: &mut TestStruct, value: i64){
        element.set_field(value);
    }
}