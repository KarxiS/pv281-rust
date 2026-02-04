use kmf_protocol::{AsyncStream, Packet};
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

/// Sends a file to a client using the two-packet protocol (DropSend + Data).
///
/// # Arguments
///
/// * `socket` - The stream to send the file over
/// * `path` - The file path to send
///
/// # Returns
///
/// - `Ok(())` if the file was sent and acknowledged
/// - `Err` if the file couldn't be read, sent, or wasn't acknowledged
pub async fn send_file<S: AsyncStream>(
    socket: &mut S,
    path: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file_path = Path::new(path);
    let filename = file_path
        .file_name()
        .ok_or("Invalid filename")?
        .to_string_lossy()
        .to_string();

    let data = read_file(path).await?;

    kmf_protocol::send(
        Packet::DropSend {
            filename: filename.clone(),
        },
        socket,
    )
    .await?;

    kmf_protocol::send(Packet::Data(data.clone()), socket).await?;

    println!("[DEBUG] File sent: {} ({} bytes)", filename, data.len());

    match kmf_protocol::receive(socket).await {
        Ok(Packet::Ok) => {
            println!("[DEBUG] File transfer acknowledged");
            Ok(())
        }
        Ok(Packet::Err {
            code: _,
            message: _,
        }) => Err("Client error".into()),
        Err(e) => Err(format!("Failed to receive ack: {}", e).into()),
        _ => Err("Unexpected response".into()),
    }
}

/// Receives a file from the server and saves it.
///
/// # Arguments
///
/// * `socket` - The stream to receive the file from
/// * `filename` - The filename to save as
///
/// # Returns
///
/// - `Ok(())` if the file was received and saved successfully
/// - `Err` if receiving or saving failed
pub async fn receive_file<S: AsyncStream>(
    socket: &mut S,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match kmf_protocol::receive(socket).await {
        Ok(Packet::Data(data)) => {
            save_file(filename, &data).await?;
            println!("[FILE] Saved: {} ({} bytes)", filename, data.len());
            kmf_protocol::send(Packet::Ok, socket).await?;
            Ok(())
        }
        Ok(other) => Err(format!("Expected Data packet, got: {:?}", other).into()),
        Err(e) => Err(format!("Failed to receive file data: {}", e).into()),
    }
}

/// Reads an entire file into memory asynchronously.
pub async fn read_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(path).await?;
    let mut data = Vec::new();
    file.read_to_end(&mut data).await?;
    Ok(data)
}

/// Saves data to a file asynchronously.
pub async fn save_file(filename: &str, data: &[u8]) -> Result<(), std::io::Error> {
    use tokio::io::AsyncWriteExt;
    let mut file = File::create(filename).await?;
    file.write_all(data).await?;
    file.flush().await?;
    Ok(())
}
