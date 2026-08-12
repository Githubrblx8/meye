//! M'Eye SMTP Gateway

use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tracing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    let port = std::env::var("SMTP_PORT")
        .unwrap_or_else(|_| "2525".to_string())
        .parse::<u16>()?;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;
    
    tracing::info!("📧 SMTP gateway listening on port {}", port);

    loop {
        let (socket, peer_addr) = listener.accept().await?;
        tracing::debug!("New connection from {}", peer_addr);
        
        tokio::spawn(handle_connection(socket, peer_addr));
    }
}

async fn handle_connection(
    socket: tokio::net::TcpStream,
    peer_addr: SocketAddr,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    // Send greeting
    writer.write_all(b"220 m-eye SMTP Gateway Ready\r\n").await?;

    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;
        
        if bytes_read == 0 {
            break; // Connection closed
        }

        let command = line.trim();
        tracing::debug!("Received: {}", command);

        let response = process_command(command).await;
        writer.write_all(response.as_bytes()).await?;

        if command.starts_with("QUIT") {
            break;
        }
    }

    tracing::debug!("Connection closed: {}", peer_addr);
    Ok(())
}

async fn process_command(command: &str) -> String {
    let parts: Vec<&str> = command.split_whitespace().collect();
    
    if parts.is_empty() {
        return "500 Error: bad syntax\r\n".to_string();
    }

    match parts[0].to_uppercase().as_str() {
        "EHLO" | "HELO" => {
            format!("250-m-eye.smtp.gateway\r\n250-SIZE 10485760\r\n250 OK\r\n")
        }
        "MAIL" => "250 OK\r\n".to_string(),
        "RCPT" => "250 OK\r\n".to_string(),
        "DATA" => {
            // In a real implementation, parse email and run through risk engine
            "354 Start mail input; end with <CRLF>.<CRLF>\r\n".to_string()
        }
        "QUIT" => "221 Bye\r\n".to_string(),
        "NOOP" => "250 OK\r\n".to_string(),
        "RSET" => "250 OK\r\n".to_string(),
        _ => "502 Error: command not implemented\r\n".to_string(),
    }
}
