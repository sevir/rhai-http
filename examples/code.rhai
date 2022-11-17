let http = new_http();

let response = http.get("https://www.digio.es");
print(response.code);
print(response.body);