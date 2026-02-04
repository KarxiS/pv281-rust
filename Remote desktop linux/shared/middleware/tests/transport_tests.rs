//! Integration tests for protocol transport (TCP and QUIC)

use kmf_protocol::transport::{TransportFactory, TransportType};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn free_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

#[tokio::test]
async fn test_tcp_transport_echo() {
    let addr = format!("127.0.0.1:{}", free_port());
    let msg = b"hello tcp!";

    // Start server
    let server_addr = addr.clone();
    let server = tokio::spawn(async move {
        let mut listener = TransportFactory::bind_server(TransportType::Tcp, &server_addr)
            .await
            .unwrap();
        let (mut stream, _peer) = listener.accept().await.unwrap();
        let mut buf = vec![0u8; 32];
        let n = stream.read(&mut buf).await.unwrap();
        stream.write_all(&buf[..n]).await.unwrap();
    });

    // Client
    let mut client = loop {
        match TransportFactory::connect_client(TransportType::Tcp, &addr).await {
            Ok(c) => break c,
            Err(_) => tokio::time::sleep(std::time::Duration::from_millis(20)).await,
        }
    };
    client.write_all(msg).await.unwrap();
    let mut buf = vec![0u8; 32];
    let n = client.read(&mut buf).await.unwrap();
    assert_eq!(&buf[..n], msg);
    server.await.unwrap();
}

#[tokio::test]
async fn test_quic_transport_echo() {
    let addr = format!("127.0.0.1:{}", free_port());
    let msg = b"hello mod!";

    // Start server
    let server_addr = addr.clone();
    let server = tokio::spawn(async move {
        let mut listener = TransportFactory::bind_server(TransportType::Quic, &server_addr)
            .await
            .unwrap();
        let (mut stream, _peer) = listener.accept().await.unwrap();
        let mut buf = vec![0u8; 32];
        let n = stream.read(&mut buf).await.unwrap();
        stream.write_all(&buf[..n]).await.unwrap();
        stream.shutdown().await.unwrap();
        let mut tmp = [0u8; 1];
        let _ = stream.read(&mut tmp).await;
    });

    // Client
    let mut client = TransportFactory::connect_client(TransportType::Quic, &addr)
        .await
        .unwrap();
    client.write_all(msg).await.unwrap();
    let mut buf = vec![0u8; 32];
    let n = client.read(&mut buf).await.unwrap();
    assert_eq!(&buf[..n], msg);
    server.await.unwrap();
}

#[tokio::test]
async fn test_quic_file_transfer() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let addr = format!("127.0.0.1:{}", free_port());
    let file_data = b"this is a test file sent over mod!".repeat(1024); // ~32KB

    // Server (slave): receive file
    let value = file_data.clone();
    let server_addr = addr.clone();
    let server = tokio::spawn(async move {
        let mut listener = TransportFactory::bind_server(TransportType::Quic, &server_addr)
            .await
            .unwrap();
        let (mut stream, _peer) = listener.accept().await.unwrap();
        let mut received = Vec::with_capacity(file_data.len());
        let mut buf = [0u8; 4096];
        let mut total = 0;
        while total < file_data.len() {
            let n = stream.read(&mut buf).await.unwrap();
            if n == 0 {
                break;
            }
            received.extend_from_slice(&buf[..n]);
            total += n;
        }
        assert_eq!(received, file_data);
    });

    // Client (master): send file
    let mut client = TransportFactory::connect_client(TransportType::Quic, &addr)
        .await
        .unwrap();
    client.write_all(&value).await.unwrap();
    client.shutdown().await.unwrap();
    server.await.unwrap();
}
