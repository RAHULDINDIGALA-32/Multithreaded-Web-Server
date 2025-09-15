use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // listens on localhost 127.0.0.1 at port 7878 

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}