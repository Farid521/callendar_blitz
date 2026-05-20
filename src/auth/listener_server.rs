use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

fn create_tcp_server(host: &str, port: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("{host}:{port}"))?;

    for stream in listener.incoming() {
        let client_stream = stream?;
        handle_connection(client_stream)?;
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut collect_buf = Vec::new();
    let buf_reader = BufReader::new(&stream);

    for line in buf_reader.lines() {
        let stream = line?;
        if stream.is_empty() {
            break;
        }
        collect_buf.push(stream);
    }

    Ok(())
}