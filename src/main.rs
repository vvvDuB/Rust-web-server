use std::fs;
use std::io::{BufRead, BufReader, Write}; 
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
    let http_request: Vec<_> = reader.lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    let resource = http_request
        .get(0)                   
        .map(|line| line.split_whitespace().nth(1).unwrap_or("/"))
        .unwrap_or("/");          
           
    match resource.trim_start_matches('/') {
        "" | "index.html" => render_page(stream, "index.html"),
        "flag.html" => render_page(stream, "flag.html"),
        "favicon.io" => render_page(stream, "favicon.io"),
        _ => {
            let resp = format!("HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n");
            stream.write_all(resp.as_bytes()).unwrap();
        },
    }

    // TODO: Add event log in console
    // TODO: Prevent path traversal
    // TODO: Handles malformed requests
}

