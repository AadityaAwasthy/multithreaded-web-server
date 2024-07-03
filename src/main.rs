use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, fs};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("The server could not bind to the specified address");

    for stream in listener.incoming() {
        let stream = stream.expect("Error while opening a connection");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(& mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("Request: {stream:#?}");

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(format!("./html/{filename}")).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

