# whereismybin

Locates installed executable inside directories specified in the *PATH*
environment variable.

The goals of this implementations are:

* No dependencies except the standard library.
* No unsafe code, no syscalls, no calls to libc.
* Compatibility with [Miri](https://github.com/rust-lang/miri).

## Example

**whereismybin** can be used as a library. Example:

```rust
use std::process::Command;

use whereismybin::whereismybin;

let my_tool = whereismybin("my-tool").expect("Could not find my-tool");
let output = Command::new(my_tool)
    .arg("--some-arg")
    .output()
    .expect("Failed to execute process");
let content = output.stdout;
```

## Why not which-rs?

The [which-rs](https://crates.io/crates/which) crate is great, but
unfortunately it doesn't run on [Miri](https://github.com/rust-lang/miri)
and depends on [libc](https://crates.io/crates/libc) crate. **whereismybin**
aims to provide more lightweight alternative.

License: Apache-2.0
