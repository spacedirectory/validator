#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate spaceapi_validator;

use std::collections::BTreeMap;
use nickel::status::StatusCode;
use nickel::{Nickel, JsonBody, HttpRouter, MediaType};
use rustc_serialize::json::{Json, ToJson, encode};

#[derive(RustcDecodable, RustcEncodable)]
struct ValidationRequest {
    version: String,
    schema:  String,
}

impl ToJson for ValidationRequest {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();
        map.insert("version".to_string(), self.version.to_json());
        map.insert("schema" .to_string(), self.schema .to_json());
        Json::Object(map)
    }
}

#[derive(RustcDecodable, RustcEncodable)]
struct Result {
    version: String,
    status: String,
    message: Option<String>,
}

fn main() {
    let mut server = Nickel::new();

    // try it with curl
    // curl 'http://localhost:6767/' -H 'Content-Type: application/json;charset=UTF-8'  --data-binary $'{"version":"0.13", "schema":"{...}"}'
    server.post("/", middleware! { |request, mut response|
        set_headers(&mut response);

        let vr = try_with!(response, {
            request.json_as::<ValidationRequest>().map_err(|e| (StatusCode::BadRequest, e))
        });

        let result = spaceapi_validator::validate_spaceapi_json(&*vr.schema);
        let result = match result {
          Ok(state) => Result {
            version: "TODO".into(),
            status: "OK".into(),
            message: None,
          },
          Err(boxed_error) => {
            let err_msg: &str = (*boxed_error).description();
            Result {
              version: "TODO".into(),
              status: "ERROR".into(),
              message: Some(err_msg.into()),
            }
          },
        };

        //format!("Hello {} {}", person.version, person.schema)
        encode(&result).unwrap()
    });


    // Supporting CORS
    server.get("/", middleware! { |request, mut response|
        set_headers(&mut response);

        "OK"
    });

    server.options("/", middleware! { |request, mut response|
        set_headers(&mut response);

        "OK"
    });

    server.listen("127.0.0.1:6767").unwrap();
}

fn set_headers(response: &mut nickel::Response) {
    response.headers_mut().set_raw("Access-Control-Allow-Origin", vec![b"*".to_vec()]);
    response.headers_mut().set_raw("Access-Control-Allow-Methods", vec![b"GET, POST, OPTIONS".to_vec()]);
    response.headers_mut().set_raw("Access-Control-Allow-Headers", vec![b"Origin, Content-Type, X-Auth-Token".to_vec()]);

    response.headers_mut().set_raw("Cache-Control", vec![b"no-cache".to_vec()]);
}
