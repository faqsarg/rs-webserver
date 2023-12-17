use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use rswebserver::ThreadPool;

fn main() {
    let pool = ThreadPool::build(4).unwrap_or_else(|e| {
        eprintln!("Failed to create thread pool: {}", e);
        std::process::exit(1);
    });
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    println!("listening on port 8000");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_conn(stream);
        });
    }
}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, contents) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", fs::read_to_string("hello.html").unwrap()),
        _ => (
            "HTTP/1.1 404 NOT FOUND",
            fs::read_to_string("404.html").unwrap(),
        ),
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
