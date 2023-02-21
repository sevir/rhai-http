use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use rhai::def_package;
use rhai::{ImmutableString, plugin::*};
use serde_json;


#[derive(Debug, Clone)]
pub struct Http {
    headers: reqwest::header::HeaderMap,
    body: String
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    code: i64,
    body: ImmutableString
}

impl Http {
    pub fn new() -> Self {
        Self { headers: HeaderMap::new(), body: String::new() }
    }

    fn get(&mut self, url: ImmutableString) -> HttpResponse {
        let response = reqwest::blocking::get(url.as_str());

        match response{
            Ok(response) => {
                let code:i64 = response.status().as_u16().into();
                let body: ImmutableString = response.text().unwrap().into();
        
                HttpResponse { code: code, body: body.clone() }
            },
            Err(_e) => {
                HttpResponse { code: 500, body: format!("{} can't be loaded", url.as_str()).into()}
            }
        }        
    }

    fn set_header(&mut self, header_name : ImmutableString, header_value : ImmutableString) -> bool{
        let header_name_owned = header_name.into_owned();
        let header_value_owned = header_value.into_owned();
        self.headers.insert(HeaderName::try_from(header_name_owned).unwrap(), HeaderValue::try_from(header_value_owned).unwrap());
        true
    }

    fn set_body(&mut self, body: ImmutableString) -> bool{
        self.body = body.into_owned().clone();
        true
    }

    fn post(&mut self, url: ImmutableString) -> HttpResponse {
        let client = reqwest::blocking::Client::new();
        let headers = self.headers.clone();
        
        let response = client.post(url.as_str())
            .headers(headers)
            .body(self.body.clone())
            .send();

        match response{
            Ok(response) => {
                let code:i64 = response.status().as_u16().into();
                let body: ImmutableString = response.text().unwrap().into();
        
                HttpResponse { code: code, body: body.clone()}
            },
            Err(_e) => {
                HttpResponse { code: 500, body: format!("{} can't be loaded", url.as_str()).into()}
            }
        }
    }
}

impl HttpResponse {
    fn code(&mut self) -> i64 {
        self.code.clone()
    }

    fn body(&mut self) -> ImmutableString {
        self.body.clone()
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

    #[rhai_fn(name = "get")]
    pub fn get(element: &mut Http, url: ImmutableString) -> HttpResponse {
        element.get(url)
    }

    #[rhai_fn(name = "post")]
    pub fn post(element: &mut Http, url: ImmutableString) -> HttpResponse {
        element.post(url)
    }

    #[rhai_fn(name = "set_header")]
    pub fn set_http_header(element: &mut Http, header_name: ImmutableString, header_value: ImmutableString) -> bool {
        element.set_header(header_name, header_value)
    }

    #[rhai_fn(name = "set_body")]
    pub fn set_http_body(element: &mut Http, body: ImmutableString) -> bool {
        element.set_body(body)
    }

    #[rhai_fn(get = "code", pure)]
    pub fn get_code(element: &mut HttpResponse) -> rhai::INT {
        element.code()
    }

    #[rhai_fn(get = "body", pure)]
    pub fn get_body(element: &mut HttpResponse) -> ImmutableString {
        element.body()
    }
    
    #[rhai_fn(name = "serialize")]
    pub fn serialize(object: rhai::Dynamic) -> ImmutableString {
        let result = serde_json::to_string(&object);
        result.unwrap().as_str().into()
    }

    #[rhai_fn(name = "parse")]
    pub fn parse(json: ImmutableString) -> Dynamic {
        let result: Dynamic = serde_json::from_str(&json.as_str()).unwrap();
        result
    }

}