use futures_core::stream::Stream;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct StreamifyAny<T: Clone> {
  inner: T,
  completed: bool,
}

impl<I: Clone> Unpin for StreamifyAny<I> {}

impl<T: Clone> StreamifyAny<T> {
  pub fn new(inner: T) -> Self {
    StreamifyAny {
      inner,
      completed: false,
    }
  }
}

impl<T: Clone> Stream for StreamifyAny<T> {
  type Item = T;

  fn poll_next(self: Pin<&mut Self>, _: &mut Context) -> Poll<Option<Self::Item>> {
    if self.completed {
      Poll::Ready(None)
    } else {
      Poll::Ready(Some(self.inner.clone()))
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (1, Some(1))
  }
}

pub fn from_any<T: Clone>(f: T) -> StreamifyAny<T> {
  StreamifyAny::new(f)
}
