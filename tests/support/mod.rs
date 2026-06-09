use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::Arc,
    thread,
};

pub struct Response {
    status: u16,
    content_type: &'static str,
    body: String,
}

impl Response {
    pub fn new(content_type: &'static str, body: impl Into<String>) -> Self {
        Self {
            status: 200,
            content_type,
            body: body.into(),
        }
    }

    pub fn error(status: u16, body: impl Into<String>) -> Self {
        Self {
            status,
            content_type: "text/plain",
            body: body.into(),
        }
    }
}

pub fn server(
    requests: usize,
    handler: impl Fn(&str, &str) -> Response + Send + Sync + 'static,
) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let base_url = format!("http://{}/", listener.local_addr().unwrap());
    let thread_url = base_url.clone();
    let handler = Arc::new(handler);
    thread::spawn(move || {
        for stream in listener.incoming().take(requests) {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 8192];
            let size = stream.read(&mut buffer).unwrap();
            let request = String::from_utf8_lossy(&buffer[..size]);
            let target = request
                .lines()
                .next()
                .and_then(|line| line.split_whitespace().nth(1))
                .unwrap();
            let response = handler(target, &thread_url);
            let reason = if response.status == 200 {
                "OK"
            } else {
                "Error"
            };
            write!(
                stream,
                "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                response.status,
                reason,
                response.content_type,
                response.body.len(),
                response.body
            )
            .unwrap();
            stream.flush().unwrap();
        }
    });
    base_url
}
