mod server;

fn main() {
    // paramiter is the file to serve
    server::start_server("test_file.txt");
}
