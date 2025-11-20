use std::fs;
use std::io::{BufRead, BufReader, Write}; 
use chrono::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let address = "127.0.0.1:8001";
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_stream(stream);
    }
}

fn render_page(mut stream: TcpStream, page: &str) {
    // TODO: Manage safe file open with 200 or 500

    let status_line = "HTTP/1.1 200 OK";
    let content = fs::read_to_string(page).unwrap();
    let len = content.len();

    let resp = format!("{status_line}\r\nContent-Length: {len}\r\n\r\n{content}");
    stream.write_all(resp.as_bytes()).unwrap();
}


fn handle_stream(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let peer = &reader.get_ref().peer_addr().unwrap().ip();

    let http_request: Vec<_> = reader.lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let header = &http_request;
    let resource = http_request
        .get(0)                   
        .map(|line| line.split_whitespace().nth(1).unwrap_or("/"))
        .unwrap_or("/");          
    
    match resource.trim_start_matches('/') {
        "" | "index.html" => render_page(stream, "index.html"),
        "favicon.io" => render_page(stream, "favicon.io"),
        _ => {
            let resp = format!("HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n");
            stream.write_all(resp.as_bytes()).unwrap();
        },
    }
   
    let date = Local::now().format("%d/%m/%y - %H:%M:%S").to_string();
    println!("{} -- {:?} {} - {}", peer, date, header[0], header[2]);

    // TODO: Prevent path traversal
    // TODO: Handles malformed requests
}

