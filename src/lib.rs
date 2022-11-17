use rhai::def_package;
use rhai::{ImmutableString, plugin::*};
#[derive(Debug, Clone)]
pub struct Http {
    cookie: ImmutableString
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    code: i64,
    body: ImmutableString,
    cookie: ImmutableString,
    headers: ImmutableString
}

impl Http {
    pub fn new() -> Self {
        Self { cookie: "".into() }
    }

    fn get_cookie(&mut self) -> ImmutableString {
        self.cookie.clone()
    }

    fn get(&mut self, url: ImmutableString) -> HttpResponse {
        let response: reqwest::blocking::Response = reqwest::blocking::get(url.as_str()).unwrap();
        let code:i64 = response.status().as_u16().into();
        let body: ImmutableString = response.text().unwrap().into();

        HttpResponse { code: code, body: body.clone(), cookie: "".into(), headers: "".into() }
    }
}

impl HttpResponse {
    fn code(&mut self) -> i64 {
        self.code.clone()
    }

    fn body(&mut self) -> ImmutableString {
        self.body.clone()
    }

    fn headers(&mut self) -> ImmutableString {
        self.headers.clone()
    }

    fn cookie(&mut self) -> ImmutableString {
        self.cookie.clone()
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

    // Http Struct
    #[rhai_fn(name = "http")]
    pub fn new_http() -> Http {
        Http::new()
    }

    #[rhai_fn(get = "cookie", pure)]
    pub fn get_cookie(element: &mut Http) -> ImmutableString {
        element.get_cookie()
    }

    #[rhai_fn(name = "get")]
    pub fn get(element: &mut Http, url: ImmutableString) -> HttpResponse {
        element.get(url)
    }

    #[rhai_fn(get = "code", pure)]
    pub fn get_code(element: &mut HttpResponse) -> rhai::INT {
        element.code()
    }

    #[rhai_fn(get = "body", pure)]
    pub fn get_body(element: &mut HttpResponse) -> ImmutableString {
        element.body()
    }

    #[rhai_fn(get = "headers", pure)]
    pub fn get_headers(element: &mut HttpResponse) -> ImmutableString {
        element.headers()
    }

    #[rhai_fn(get = "cookie", pure)]
    pub fn get_response_cookie(element: &mut HttpResponse) -> ImmutableString {
        element.cookie()
    }
}