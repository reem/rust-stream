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
    #[inline]
    pub fn head(&self) -> &T { &*self.head }

    /// Get the rest of the stream, skipping the first value.
    #[inline]
    pub fn tail(&self) -> Arc<Stream<T>> { self.tail.deref().clone() }

    /// Get an infinite iterator over the contents of the stream.
    #[inline]
    pub fn iter<'a>(&'a self) -> StreamIter<'a, T> {
        StreamIter { stream: self }
    }
}

/// An iterator of references over the contents of a stream.
pub struct StreamIter<'a, T> { stream: &'a Stream<T> }

impl<'a, T: Send + Share> Iterator<&'a T> for StreamIter<'a, T> {
    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        let result = self.stream.head();
        self.stream = &**self.stream.tail;
        Some(result)
    }
}

