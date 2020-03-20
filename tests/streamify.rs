use futures::executor::block_on;
use futures::stream::StreamExt;
use std::future::Future;
use streamify;

#[test]
fn streamify_a_future() {
  // Test Copy values
  let fut = async { 11 };
  let mut stream = streamify::from_future(fut);

  block_on(async {
    assert_eq!(Some(11), stream.next().await);
    assert_eq!(None, stream.next().await);
  });

  // Test Non-Copy values
  let fut = async { "Rust".to_owned() };
  let mut stream = streamify::from_future(fut);

  block_on(async {
    assert_eq!(Some("Rust".to_owned()), stream.next().await);
    assert_eq!(None, stream.next().await);
  });
}

#[test]
fn random() {
  let x = async { 10 };
  let y = async { 33 };

  // let f: &'static dyn Future<Output = i32> = &x;

  let arr: [&dyn Future<Output = i32>; 2] = [&x, &y];
  let mut iter = arr.iter().copied();
  let y = iter.next();
}
