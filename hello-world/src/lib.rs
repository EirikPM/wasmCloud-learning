wit_bindgen::generate!({ generate_all });

use exports::wasi::http::incoming_handler::Guest;
use wasi::http::types::*;

struct HttpServer;

impl Guest for HttpServer {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(200).unwrap();
        let response_body = response.body().unwrap();

        let name = extract_name_from_query(&request);

        ResponseOutparam::set(response_out, Ok(response));
        response_body
            .write()
            .unwrap()
            .blocking_write_and_flush(format!("Hello, {}!\n", name).as_bytes())
            .unwrap();
        OutgoingBody::finish(response_body, None).expect("failed to finish response body");
    }
}

fn extract_name_from_query(request: &IncomingRequest) -> String {
    match request
        .path_with_query()
        .unwrap()
        .split("=")
        .collect::<Vec<&str>>()[..]
    {
        ["/?name", name] => name.to_string(),
        _ => "World".to_string(),
    }
}

export!(HttpServer);
