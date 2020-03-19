# Streamify

Convert a Rust value to a Stream. [WIP].


## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
streamify = "0.1.0"
```

## Examples

Convert a `Future` to a `Stream`:

```rust
use streamify;

fn main() {
  let fut = async { 10 };
  let stream = streamify::from_future(fut);
}
```

Convert any value to a `Stream`:

```rust
use streamify;

fn main() {
  let val = 10;
  let stream = streamify::from_any(val);

  let val = "Testing".to_owned();
  let stream = streamify::from_any(val);
}
```