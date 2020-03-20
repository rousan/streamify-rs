use futures_core::stream::Stream;
use std::future::Future;
use std::marker::Unpin;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct StreamifyFuture<T: Future> {
  inner: T,
  completed: bool,
}

impl<I: Future> Unpin for StreamifyFuture<I> {}

impl<T: Future> StreamifyFuture<T> {
  pub fn new(inner: T) -> Self {
    StreamifyFuture {
      inner,
      completed: false,
    }
  }
}

impl<T: Future> Stream for StreamifyFuture<T> {
  type Item = <T as Future>::Output;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
    if self.completed {
      return Poll::Ready(None);
    }

    let pinned_future = unsafe { Pin::new_unchecked(&mut self.inner) };

    let status = pinned_future.poll(cx);
    if let Poll::Ready(val) = status {
      self.completed = true;
      Poll::Ready(Some(val))
    } else {
      Poll::Pending
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (1, Some(1))
  }
}

pub fn from_future<T: Future>(f: T) -> StreamifyFuture<T> {
  StreamifyFuture::new(f)
}

// pub fn from_futures<T, I: IntoIterator<Item = Box<dyn Future<Output = T>>>>(i: I) {
//   for item in i {
//     from_future(item);
//   }
// }

// 2. streamify::from_futures(fut); IntoIterator<Item = impl Future<Output = T>>
