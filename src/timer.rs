use futures::{Future, Async, Poll};
use futures::unsync::oneshot::{self, Sender, Receiver};
use tokio::timer::{Delay, Error};

use std::time::Instant;

/// A cancellable timer implementation using futures. This struct internally
/// wraps around a `tokio::timer::Delay` instance. The timer can be cancelled
/// by calling `send` on the provided `Sender`. The future contains `true` when
/// the `Delay` finished and `false` when the timer was cancelled.
#[derive(Debug)]
pub struct Timer {
    delay: Delay,
    cancel: Receiver<()>,
}

impl Timer {
    /// Returns a new `Timer` instance and a `Sender` to cancel the timer.
    pub fn new(finish_at: Instant) -> (Timer, Sender<()>) {
        let (tx, rx) = oneshot::channel();
        (Timer { delay: Delay::new(finish_at), cancel: rx }, tx)
    }
}

impl Future for Timer {
    type Item = bool;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.cancel.poll() {
            Ok(Async::Ready(())) => return Ok(Async::Ready(false)),
            Ok(Async::NotReady) => {},
            Err(e) => eprintln!("Sender was droppped: {}", e),
        }
        match self.delay.poll() {
            Ok(Async::Ready(())) => Ok(Async::Ready(true)),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => Err(e),
        }
    }
}

// just for debug purposes
impl Drop for Timer {
    fn drop(&mut self) {
        println!("Timer was dropped.");
    }
}
