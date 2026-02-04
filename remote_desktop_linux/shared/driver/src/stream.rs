use std::io::Result;

use core::task::{Context, Poll};
use evdev::uinput::{VirtualDevice, VirtualEventStream};
use evdev::{Device, EventStream, InputEvent};

use futures_core::Stream;

pub trait InputStream: Stream<Item = Result<InputEvent>> {
    fn get_next_event(&mut self) -> impl std::future::Future<Output = Result<InputEvent>> + Send;
    fn poll_next_event(&mut self, cx: &mut Context<'_>) -> Poll<Result<InputEvent>>;
}

pub trait ToStream {
    type Stream: InputStream;

    fn to_stream(self) -> Result<Self::Stream>;
    fn get_events(&mut self) -> Result<Vec<InputEvent>>;
    fn ungrab_device(&mut self) -> Result<()>;
}

impl InputStream for VirtualEventStream {
    async fn get_next_event(&mut self) -> Result<InputEvent> {
        self.next_event().await
    }

    fn poll_next_event(&mut self, cx: &mut Context<'_>) -> Poll<Result<InputEvent>> {
        self.poll_event(cx)
    }
}

impl InputStream for EventStream {
    async fn get_next_event(&mut self) -> Result<InputEvent> {
        self.next_event().await
    }

    fn poll_next_event(&mut self, cx: &mut Context<'_>) -> Poll<Result<InputEvent>> {
        self.poll_event(cx)
    }
}

impl ToStream for VirtualDevice {
    type Stream = VirtualEventStream;
    fn to_stream(self) -> Result<Self::Stream> {
        self.into_event_stream()
    }

    fn get_events(&mut self) -> Result<Vec<InputEvent>> {
        Ok(self.fetch_events()?.collect())
    }

    fn ungrab_device(&mut self) -> Result<()> {
        Ok(())
    }
}

impl ToStream for Device {
    type Stream = EventStream;
    fn to_stream(self) -> Result<Self::Stream> {
        self.into_event_stream()
    }

    fn get_events(&mut self) -> Result<Vec<InputEvent>> {
        Ok(self.fetch_events()?.collect())
    }

    fn ungrab_device(&mut self) -> Result<()> {
        self.ungrab()
    }
}
