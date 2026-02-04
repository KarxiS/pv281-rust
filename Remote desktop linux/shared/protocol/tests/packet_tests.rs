use kmf_protocol::serialization::{read_data_payload, read_packet_type};
use kmf_protocol::{PacketType, ProtocolError};
use tokio::io::{duplex, AsyncWriteExt};

#[tokio::test]
async fn test_read_packet_type_ok() {
    // PacketType::Action == 3
    let (mut a, mut b) = duplex(64);
    a.write_all(&[3u8]).await.unwrap();

    let res = read_packet_type(&mut b).await;
    let pkt = res.expect("should read packet type");
    assert_eq!(pkt, PacketType::Action);
}

#[tokio::test]
async fn test_read_data_payload_ok() {
    let (mut a, mut b) = duplex(128);
    let payload = b"hello";
    let len = (payload.len() as u32).to_be_bytes();
    // write length prefix + payload
    a.write_all(&len).await.unwrap();
    a.write_all(payload).await.unwrap();

    let data = read_data_payload(&mut b)
        .await
        .expect("should read payload");
    assert_eq!(data, payload);
}

#[tokio::test]
async fn test_read_data_payload_truncated() {
    let (mut a, mut b) = duplex(16);
    // claim length 10, but provide only 3 bytes
    let len = (10u32).to_be_bytes();
    a.write_all(&len).await.unwrap();
    a.write_all(&[1u8, 2u8, 3u8]).await.unwrap();
    drop(a); // Close the writer to signal EOF

    let res = read_data_payload(&mut b).await;
    match res {
        Err(ProtocolError::Io(_)) => { /* expected an IO error */ }
        Err(e) => panic!("unexpected error kind: {:?}", e),
        Ok(_) => panic!("expected error but got Ok"),
    }
}
