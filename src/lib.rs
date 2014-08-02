#![license = "MIT"]
#![deny(missing_doc)]
#![deny(warnings)]
#![feature(phase)]

//! Infinite, lazy streams in Rust.

#[phase(plugin, link)] extern crate lazy;

use Thunk = lazy::SharedThunk;
use std::sync::Arc;

/// An infinite, immutable, shareable lazy stream
pub struct Stream<T> {
     head: Thunk<T>,
     tail: Thunk<Arc<Stream<T>>>
}

impl<T: Send + Share> Stream<T> {
    /// Get, and force evaluation of, the value at the front of the Stream.
    pub fn head(&self) -> &T { &*self.head }
    /// Get the rest of the stream, skipping the first value.
    pub fn tail(&self) -> Arc<Stream<T>> { self.tail.deref().clone() }
}

