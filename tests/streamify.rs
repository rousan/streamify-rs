use streamify;

#[test]
fn streamify_a_future() {
  let fut = async { 10 };
  let stream = streamify::from_future(fut);
}

#[test]
fn streamify_any_value() {
  let val = 10;
  let stream = streamify::from_any(val);

  let val = "Testing".to_owned();
  let stream = streamify::from_any(val);
}
