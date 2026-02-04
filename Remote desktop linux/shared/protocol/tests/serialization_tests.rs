use kmf_protocol::serialization::{
    deserialize_bin, deserialize_json, read_data_payload, read_packet_type, serialize_bin,
    serialize_json, SerializationMode,
};
use kmf_protocol::PacketType;
use kmf_protocol::{error, serialization, Packet, ProtocolError};
use serde::{Deserialize, Serialize};
use serial_test::serial;
use std::env;
use tokio::io::{duplex, AsyncWriteExt};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct TestStruct {
    a: u32,
    b: String,
}

#[test]
#[serial]
fn test_json_serialize_deserialize() {
    let value = TestStruct {
        a: 42,
        b: "hello".into(),
    };
    let data = serialize_json(&value);
    let decoded: TestStruct = deserialize_json(&data).expect("json decode");
    assert_eq!(value, decoded);
}

#[test]
#[serial]
fn test_bin_serialize_deserialize() {
    let value = TestStruct {
        a: 7,
        b: "world".into(),
    };
    let data = serialize_bin(&value);
    let decoded: TestStruct = deserialize_bin(&data).expect("bin decode");
    assert_eq!(value, decoded);
}

#[test]
#[serial]
fn test_serialization_mode_from_env_defaults_json() {
    // Ensure deterministic environment for the test
    env::set_var("PROTOCOL_SERIALIZATION", "json");
    let m = SerializationMode::from_env();
    assert_eq!(m, SerializationMode::Json);
}

#[test]
#[serial]
fn test_serialization_mode_from_env_binary() {
    env::set_var("PROTOCOL_SERIALIZATION", "binary");
    let m = SerializationMode::from_env();
    assert_eq!(m, SerializationMode::Binary);
}

#[tokio::test]
#[serial]
async fn test_read_packet_type_ok() {
    // PacketType::Action == 3
    let (mut a, mut b) = duplex(64);
    a.write_all(&[3u8]).await.unwrap();

    let res: Result<PacketType, error::ProtocolError> = read_packet_type(&mut b).await;
    let pkt = res.expect("should read packet type");
    assert_eq!(pkt, PacketType::Action);
}

#[tokio::test]
#[serial]
async fn test_read_data_payload_ok() {
    let (mut a, mut b) = duplex(128);
    let payload = b"hello";
    let len = (payload.len() as u32).to_be_bytes();
    // write length prefix + payload
    a.write_all(&len).await.unwrap();
    a.write_all(payload).await.unwrap();
    a.shutdown().await.unwrap();

    let data: Vec<u8> = read_data_payload(&mut b)
        .await
        .expect("should read payload");
    assert_eq!(data, payload);
}

#[tokio::test]
#[serial]
async fn test_read_data_payload_truncated() {
    let (mut a, mut b) = duplex(16);
    // claim length 10, but provide only 3 bytes
    let len = (10u32).to_be_bytes();
    a.write_all(&len).await.unwrap();
    a.write_all(&[1u8, 2u8, 3u8]).await.unwrap();
    a.shutdown().await.unwrap();

    let res = read_data_payload(&mut b).await;
    match res {
        Err(ProtocolError::Io(_)) => { /* expected an IO error */ }
        Err(e) => panic!("unexpected error kind: {:?}", e),
        Ok(_) => panic!("expected error but got Ok"),
    }
}

#[tokio::test]
#[serial]
async fn test_send_receive_action_both_modes() {
    // action represented as a serde_json::Value; works for both Json and Binary modes
    let action = serde_json::json!({"type": "MouseMove", "x": 123, "y": 456});
    let packet = Packet::Action(action.clone());

    for mode in &["json", "binary"] {
        std::env::set_var("PROTOCOL_SERIALIZATION", mode);

        let (mut a, mut b) = duplex(1024);
        // serialize the packet explicitly according to the selected mode
        let mode_enum = if *mode == "binary" {
            SerializationMode::Binary
        } else {
            SerializationMode::Json
        };
        let bytes = packet.serialize_with_mode(mode_enum);
        // write the complete packet bytes to the stream immediately
        a.write_all(&bytes).await.expect("write_all");
        a.shutdown().await.expect("shutdown");

        // receive should read according to the env (we set it above)
        let received = serialization::receive(&mut b).await.expect("receive ok");

        match received {
            Packet::Action(v) => assert_eq!(v, action),
            other => panic!("expected Action packet, got {:?}", other),
        }
    }
}
