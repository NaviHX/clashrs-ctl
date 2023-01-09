pub mod traffic;
pub mod log;

use std::{collections::VecDeque, marker::PhantomData, pin::Pin};

use bytes::Bytes;
use futures::Stream;
use serde::Deserialize;
use std::marker::Unpin;
use reqwest::Client;

use crate::ClashRequest;

const BUFFERSIZE: usize = 16;

pub struct ClashStream<T>
where
    T: Unpin + for<'b> Deserialize<'b>,
{
    base_stream: Pin<Box<dyn Stream<Item = reqwest::Result<Bytes>>>>,
    _marker: PhantomData<T>,

    disconnected: bool,
    buf: VecDeque<Bytes>,
}

impl<T> Stream for ClashStream<T>
where
    T: Unpin + for<'b> Deserialize<'b>,
{
    type Item = Result<T, Box<dyn std::error::Error>>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        if self.disconnected == false && self.buf.len() < BUFFERSIZE {
            // Receive the data if any or disconnect if error occurs.
            while let std::task::Poll::Ready(res) = self.as_mut().base_stream.as_mut().poll_next(cx) {
                match res {
                    Some(Ok(bytes)) => {
                        // Put the bytes received into the buffer queue
                        self.as_mut().buf.push_back(bytes);

                        if self.buf.len() >= BUFFERSIZE {
                            break;
                        }
                    }
                    None | Some(Err(_)) => {
                        // Set the stream disconnected.
                        // After this, there is no need
                        // to poll the base stream.
                        self.as_mut().disconnected = true;
                        break;
                    }
                }
            }
        }

        // Return `Pending` if no json received.
        // Or generate an object and return `Ready(T)`
        // If the base stream is disconnected, it will
        // return `Ready(T)` until all datas are consumed.
        if let Some(bytes) = self.as_mut().buf.pop_front() {
            let res = std::str::from_utf8(&bytes[..])
                .or_else(|e| Err(Box::new(e) as Box<dyn std::error::Error>))
                .and_then(|s| serde_json::from_str(s)
                          .or_else(|e| Err(Box::new(e) as Box<dyn std::error::Error>)));
            std::task::Poll::Ready(Some(res))
        } else if self.as_ref().disconnected {
            std::task::Poll::Ready(None)
        } else {
            std::task::Poll::Pending
        }
    }
}

impl<T: std::marker::Unpin + for<'a> Deserialize<'a>> ClashStream<T> {
    pub fn new(stream: Pin<Box<dyn Stream<Item = reqwest::Result<Bytes>>>>) -> Self {
        Self {
            base_stream: stream,
            _marker: PhantomData,
            disconnected: false,
            buf: VecDeque::new(),
        }
    }
}

async fn get_stream_request(request: impl ClashRequest) -> Result< Pin<Box<dyn Stream<Item = reqwest::Result<Bytes>>>>, Box<dyn std::error::Error> > {
    let mut c = Client::new()
        .get(format!("http://{}:{}/{}?{}",
                     request.get_dest(),
                     request.get_port(),
                     request.get_path(),
                     request.get_query_parameter()))
        .body(request.get_body());
    if let Some(secret) = request.get_secret() {
        c = c.header("Authorization", format!("Bearer {}", secret));
    }

    let c = c.send()
        .await?
        .bytes_stream();

    Ok(Box::pin(c))
}

