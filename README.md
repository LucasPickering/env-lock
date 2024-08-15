# env-lock

[![Test CI](https://github.com/github/docs/actions/workflows/test.yml/badge.svg)](https://github.com/LucasPickering/env-lock/actions)
[![crates.io](https://img.shields.io/crates/v/env-lock.svg)](https://crates.io/crates/env-lock)
[![docs.rs](https://img.shields.io/docsrs/env-lock)](https://docs.rs/env-lock)

A process's environment is a form of global mutable state. In Rust, tests are run in a shared process. This means tests that modify environment variables can inadvertently affect each other. `env-lock` provides an interface to safely modify and lock the process environment, to prevent simultaneous access.

```rust
use std::env;

let var = "ENV_LOCK_TEST_VARIABLE";
assert!(env::var(var).is_err());

let guard = env_lock::lock_env([(var, Some("hello!"))]);
assert_eq!(env::var(var).unwrap(), "hello!");
drop(guard);

assert!(env::var(var).is_err());
```
