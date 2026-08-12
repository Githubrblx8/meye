use sqlx::postgres::PgPool;
use tokio::net::TcpListener;
use tracing::{error, info};

/// Start the SMTP gateway listener
pub async fn start_smtp_gateway(db: PgPool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let smtp_port = std::env::var("SMTP_PORT").unwrap_or_else(|_| "2525".to_string());
    let addr = format!("0.0.0.0:{}", smtp_port);

    let listener = TcpListener::bind(&addr).await?;
    info!("SMTP gateway listening on {}", addr);

    loop {
        match listener.accept().await {
            Ok((socket, peer_addr)) => {
                info!("New SMTP connection from {}", peer_addr);
                let db_clone = db.clone();
                
                tokio::spawn(async move {
                    if let Err(e) = handle_smtp_connection(socket, db_clone).await {
                        error!("Error handling SMTP connection: {}", e);
                    }
                });
            }
            Err(e) => {
                error!("Failed to accept SMTP connection: {}", e);
            }
        }
    }
}

/// Handle individual SMTP connections
async fn handle_smtp_connection(
    socket: tokio::net::TcpStream,
    _db: PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    
    // Send greeting
    writer.write_all(b"220 M'Eye SMTP Gateway Ready\r\n").await?;
    
    let mut line = String::new();
    
    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => break, // Connection closed
            Ok(_) => {
                let command = line.trim().to_uppercase();
                
                // Simple SMTP command handling
                if command.starts_with("EHLO") || command.starts_with("HELO") {
                    writer.write_all(b"250-m-eye.local\r\n").await?;
                    writer.write_all(b"250 SIZE 10485760\r\n").await?;
                } else if command.starts_with("MAIL FROM:") {
                    writer.write_all(b"250 OK\r\n").await?;
                } else if command.starts_with("RCPT TO:") {
                    writer.write_all(b"250 OK\r\n").await?;
                } else if command.starts_with("DATA") {
                    writer.write_all(b"354 Start mail input\r\n").await?;
                    // In a real implementation, we would read the email content here
                    // and pass it to the analysis pipeline
                    line.clear();
                    loop {
                        line.clear();
                        reader.read_line(&mut line).await?;
                        if line.trim() == "." {
                            break;
                        }
                    }
                    writer.write_all(b"250 OK: Message accepted\r\n").await?;
                } else if command.starts_with("QUIT") {
                    writer.write_all(b"221 Bye\r\n").await?;
                    break;
                } else if command.is_empty() {
                    continue;
                } else {
                    writer.write_all(b"502 Command not implemented\r\n").await?;
                }
            }
            Err(e) => {
                error!("Error reading SMTP command: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}
