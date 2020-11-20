extern crate tiny_http;

// made with the help of tinyHTTP
use tiny_http::{Response, Server};
use std::fs;

pub fn start_server(file_to_serve : &str) {
    let server = Server::http("0.0.0.0:8080").unwrap();

    println!("Started Server on port 8080");

    // requests
    for request in server.incoming_requests() {
        println!(
            "url: {:?}",
            request.url()
        );

        // 200 server responses
        let response = Response::from_string(read_file(file_to_serve));
        request.respond(response);
    }
}

// read file function
// input: filename [0]
// output: returns contents of file as std::string::String
fn read_file(file_name : &str) -> std::string::String {
    // --snip--
    let contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");

    return contents
}
