extern crate tiny_http;
use tiny_http::{Response, Server};
use tiny_http::Header;

mod util;

pub fn start_server() {
 
  const SERVER_PORT: i16 = 8080;
  let server_address: String = format!("0.0.0.0:{}", SERVER_PORT);
  let server = Server::http(server_address).unwrap();

  println!("Started Server on port :{}", SERVER_PORT);

  for request in server.incoming_requests() {
    println!(
      "url: {:?}",
      request.url()
    );

    let requested_file: &str = request.url();
    let file_contents: String = util::read_file(requested_file);
    let content_header = "Content-Type: text/html"
      .parse::<Header>()
      .unwrap();
    
    let response =
      Response::from_string(file_contents)
      .with_header(content_header);
    
    let _result = request.respond(response);
  }
}
