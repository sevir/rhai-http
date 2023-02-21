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