let http = new_http();

let response = http.get("https://www.digioooooo.es");
print(response.code);
print(response.body);