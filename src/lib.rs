mod any;
mod future;

pub use any::from_any;
pub use future::from_future;

// Functions:
// 1. streamify::from_future(fut);
// 2. streamify::from_futures(fut); IntoIterator<Item = impl Future<Output = T>>
// 3. streamify::from_any(val);
// 4. streamify::from_any_by<F: FnOnce() -> impl Future>(f: F);
// 5. streamify::from_iter()
// 6. streamify::from_streams(streams)
// 7. streamify::flatten()
