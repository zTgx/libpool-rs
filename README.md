# [libpool-rs](https://github.com/zTgx/libpool-rs.git)  [![Build Status](https://travis-ci.org/zTgx/libpool-rs.svg?branch=master)](https://travis-ci.org/zTgx/libpool-rs)
This libpool project from [TheRustProgrammingLanguageBook](https://doc.rust-lang.org/stable/book/ch20-03-graceful-shutdown-and-cleanup.html) is for study.

# Usage
Add dependencies
```
[dependencies]
libpool = "0.1.0"
```

```rust
extern crate libpool;
use libpool::*;

fn main() {
    ThreadPool::new(5).execute(move ||{
    });
}
```
