use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

pub fn wait_for_auth_code(host: &str, port: &str) -> Result<String, Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("{host}:{port}"))?;
    println!("Menunggu callback OAuth di {}:{}...", host, port);

    let (stream, _) = listener.accept()?;
    let code = handle_connection(stream)?;
    Ok(code)
}

fn handle_connection(mut stream: TcpStream) -> Result<String, Box<dyn std::error::Error>> {
    let buf_reader = BufReader::new(&stream);

    let request_line = buf_reader
        .lines()
        .next()
        .ok_or("Request kosong")??;

    println!("Request masuk: {}", request_line);

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
        <html><body><h2>Autentikasi berhasil! Silakan tutup tab ini.</h2></body></html>";
    stream.write_all(response.as_bytes())?;

    let code = request_line
        .split_whitespace()
        .nth(1) 
        .and_then(|path| {
            path.split('?')
                .nth(1) 
                .and_then(|query| {
                    query.split('&').find_map(|param| {
                        let mut parts = param.splitn(2, '=');
                        if parts.next() == Some("code") {
                            parts.next().map(|v| v.to_string())
                        } else {
                            None
                        }
                    })
                })
        })
        .ok_or("Auth code tidak ditemukan dalam callback URL")?;

    Ok(code)
}