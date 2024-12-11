use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use std::fs;
use std::path::Path;
use tokio::net::TcpStream;
use tokio::time::timeout;


pub async fn send_data_to_pico(lines: &Vec<String>) -> io::Result<()> {
    let host = "10.0.0.139";
    let port = 1234;
    let address = format!("{}:{}", host, port);
    println!(
        "Connecting to {}:{} to send {} lines",
        host,
        port,
        lines.len()
    );

    let mut stream = match timeout(Duration::from_secs(5), TcpStream::connect(&address)).await {
        Ok(Ok(stream)) => {
            println!("Successfully connected to the server!");
            stream
        }
        Ok(Err(e)) => {
            eprintln!("Connection failed: {}", e);
            return Err(e);
        }
        Err(_) => {
            eprintln!("Connection timed out.");
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "Connection timed out",
            ));
        }
    };

    use tokio::time::{sleep, Duration};

    async fn read_ack() {
        sleep(Duration::from_millis(80)).await;
    }

    //combine lines into a single string
    let lines = lines.join("\n");
    let transmission_size = lines.len();
    //send LEN: transmission size
    stream.write_all(format!("LEN:{}", transmission_size).as_bytes()).await?;

    //send the data
    stream.write_all(lines.as_bytes()).await?;

    let mut buffer = [0; 1024];
    println!("Waiting for final response...");
    let n = stream.read(&mut buffer).await?;
    println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));

    println!("Data sent successfully!");
    Ok(())
}