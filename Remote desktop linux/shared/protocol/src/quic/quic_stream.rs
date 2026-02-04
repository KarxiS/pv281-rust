use quinn::{RecvStream, SendStream};
use std::io;

/// QUIC bidirectional stream wrapper
pub struct QuinnStream {
    pub send: SendStream,
    pub recv: RecvStream,
}

impl QuinnStream {
    /// Create a new QuinnStream from quinn's SendStream and RecvStream
    pub const fn new(send: SendStream, recv: RecvStream) -> Self {
        Self { send, recv }
    }
}

impl tokio::io::AsyncRead for QuinnStream {
    fn poll_read(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        use std::pin::Pin;
        Pin::new(&mut self.recv)
            .poll_read(cx, buf)
            .map_err(io::Error::other)
    }
}

impl tokio::io::AsyncWrite for QuinnStream {
    fn poll_write(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        use std::pin::Pin;
        Pin::new(&mut self.send)
            .poll_write(cx, buf)
            .map_err(io::Error::other)
    }
    fn poll_flush(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        use std::pin::Pin;
        Pin::new(&mut self.send)
            .poll_flush(cx)
            .map_err(io::Error::other)
    }
    fn poll_shutdown(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        use std::pin::Pin;
        Pin::new(&mut self.send)
            .poll_shutdown(cx)
            .map_err(io::Error::other)
    }
}
