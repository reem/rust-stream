#![license = "MIT"]
#![deny(missing_doc)]
#![deny(warnings)]
#![feature(phase)]

//! Infinite, lazy streams in Rust.

#[phase(plugin, link)] extern crate lazy;

use lazy::SyncThunk as Thunk;
use std::sync::Arc;

/// An infinite, immutable, shareable lazy stream
pub struct Stream<T> {
     head: Thunk<T>,
     tail: Thunk<Arc<Stream<T>>>
}

impl<T: Send + Sync> Stream<T> {
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

impl<T: Send + Sync + Clone> Stream<T> {
    /// Create an infinite stream from a function and an initial input.
    /// The contents of the stream are created like so:
    ///
    /// `x`, `f(x)`, `f(f(x))`, `f(f(f(x)))`, etc.
    pub fn from_fn(x: T, f: fn(T) -> T) -> Stream<T> {
        Stream {
            head: Thunk::evaluated(x.clone()),
            tail: lazy!(Arc::new(Stream::from_fn(f(x), f)))
        }
    }
}

impl<T: Send + Sync + Copy> Stream<T> {
    /// Create an infinite stream from a function and an initial input.
    /// The contents of the stream are created like so:
    ///
    /// `x`, `f(x)`, `f(f(x))`, `f(f(f(x)))`, etc.
    ///
    /// This is a specialized version of the function for types
    /// which are copy instead of clone.
    pub fn from_fn_copy(x: T, f: fn(T) -> T) -> Stream<T> {
        Stream {
            head: Thunk::evaluated(x),
            tail: lazy!(Arc::new(Stream::from_fn_copy(f(x), f)))
        }
    }
}

/// An iterator of references over the contents of a stream.
pub struct StreamIter<'a, T> { stream: &'a Stream<T> }

impl<'a, T: Send + Sync> Iterator<&'a T> for StreamIter<'a, T> {
    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        let result = self.stream.head();
        self.stream = &**self.stream.tail;
        Some(result)
    }
}

