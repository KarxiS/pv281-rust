use tokio::io::{AsyncRead, AsyncWrite};

/// Used to allow underlying stream switching(tcp, quic, ...)
pub trait AsyncStream: AsyncRead + AsyncWrite + Unpin + Send {}

impl<T: AsyncRead + AsyncWrite + Unpin + Send> AsyncStream for T {}
