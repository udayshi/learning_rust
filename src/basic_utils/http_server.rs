use chrono::Local;
use std::io::{Read, Write};
use std::net::{IpAddr, TcpListener, TcpStream};
use std::thread;

/// Starts a basic HTTP server that logs each client's IP address and access time.
pub fn run() -> std::io::Result<()> {
    let author = "Uday Shiwakoti";
    let listener = TcpListener::bind("0.0.0.0:80")?;

    println!(
        "--- Simple HTTP Server for {} (uday@uday.com.np) ---",
        author
    );
    println!("Server running on http://127.0.0.1:80 (Press Ctrl+C to stop)");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(error) = handle_client(stream) {
                        eprintln!("Client handling failed: {error}");
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let client_ip = stream.peer_addr()?.ip();
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    if bytes_read == 0 {
        return Ok(());
    }

    println!("{}", request_log_line(client_ip, &access_time()));

    let response_time = access_time();
    let response = http_response(client_ip, &response_time);
    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    println!("{}", response_log_line(client_ip, &response_time));
    Ok(())
}

fn access_time() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

fn request_log_line(client_ip: IpAddr, accessed_at: &str) -> String {
    let log_line: String = format!("[{accessed_at}] Request received from client IP {client_ip}");
    log_line
}

fn response_log_line(client_ip: IpAddr, accessed_at: &str) -> String {
    let log_line: String = format!("[{accessed_at}] Response 200 OK sent to client IP {client_ip}");
    log_line
}

fn http_response(client_ip: IpAddr, accessed_at: &str) -> String {
    let body = format!("ok\nclient_ip: {client_ip}\naccess_time: {accessed_at}\n");
    let response: String = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    );
    response
}
