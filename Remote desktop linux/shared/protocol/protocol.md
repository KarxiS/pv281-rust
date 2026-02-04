# Communication Protocol

## Overview

This protocol enables multi-PC mouse/keyboard sharing and file transfer over TCP/IP. The server acts as the master,
broadcasting events and files to connected clients.

## State Diagram

## Packet Types

### Protocol Commands

| Command       | ID | Description                       |
|---------------|----|-----------------------------------|
| `Ok`          | 0  | Acknowledgment response           |
| `Err`         | 1  | Error response with message       |
| `ServerHello` | 2  | Initial handshake with config     |
| `Action`      | 3  | Mouse/keyboard event              |
| `ClientQuit`  | 4  | Graceful disconnect               |
| `DropSend`    | 5  | File transfer initiation          |
| `DropRequest` | 6  | Request file from client          |
| `Data`        | 7  | File data payload                 |
| `EdgeL`       | 8  | Cursor hit left edge (switch PC)  |
| `EdgeR`       | 9  | Cursor hit right edge (switch PC) |

## Packet Format

### Simple Packets (No Payload)

```
[PacketType: u8]
```

Used by: `Ok`, `ClientQuit`, `EdgeL`, `EdgeR`

### String/JSON Packets

```
[PacketType: u8][Length: u32 BE][Payload: bytes]
```

Used by: `Err`, `ServerHello`, `Action`, `DropSend`, `DropRequest`

### Data Packet

```
[PacketType: u8][Length: u32 BE][Data: bytes]
```

Used by: `Data`

## Protocol Flow

### 1. Connection Establishment

```
Client -> Server: ServerHello(config)
Server -> Client: (waits for commands)
```

### 2. Mouse/Keyboard Events

```
Server -> Client: Action(MouseMove{x, y})
Client -> Server: Ok
```

### 3. File Transfer (Server to Client)

```
Server -> Client: DropSend{filename}
Server -> Client: Data(bytes)
Client -> Server: Ok
```

### 4. File Transfer (Client to Server)

```
Server -> Client: DropRequest{filename}
Client -> Server: Data(bytes)
Server -> Client: Ok
```

### 5. Edge Detection

```
Server -> Client: EdgeL/EdgeR
Client -> Server: Ok
(Server switches active client)
```

### 6. Graceful Disconnect

```
Server -> Client: ClientQuit
Client: (closes connection)
```

## State Variables

### Server State

- `cur`: Current active client index (where mouse/keyboard is active)
- `source`: Drag source client index (-1 = no drag in progress)

### Client State

- Connected/Disconnected
- Screen dimensions
- Hostname

## Error Handling

All operations that can fail should:

1. Send `Err(message)` packet with error description
2. Log the error locally
3. Optionally close the connection for critical errors

## Implementation Notes

- All integers are big-endian (network byte order)
- JSON serialization uses `serde_json`
- TCP connections ensure reliable, ordered delivery
- No encryption (for trusted local networks only)
- Immediate flush after each packet (no buffering)
