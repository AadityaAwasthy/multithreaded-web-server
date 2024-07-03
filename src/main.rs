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
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");

    //Returning message
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("./html/hello.html").expect("Could not read the returning html file to string");
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).expect("Could not write message to stream");
}
