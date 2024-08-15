# env-guard

[![Test CI](https://github.com/github/docs/actions/workflows/test.yml/badge.svg)](https://github.com/LucasPickering/env-guard/actions)
[![crates.io](https://img.shields.io/crates/v/env-guard.svg)](https://crates.io/crates/env-guard)
[![docs.rs](https://img.shields.io/docsrs/env-guard)](https://docs.rs/env-guard)

A process's environment is a form of global mutable state. In Rust, tests are run in a shared process. This means tests that modify environment variables can inadvertently affect each other. `env-guard` provides an interface to safely modify and lock the process environment, to prevent simultaneous access.

```rust
use env_guard::EnvGuard;
use std::env;

let var = "ENV_GUARD_TEST_VARIABLE";
assert!(env::var(var).is_err());

let guard = EnvGuard::lock([(var, Some("hello!"))]);
assert_eq!(env::var(var).unwrap(), "hello!");
drop(guard);

assert!(env::var(var).is_err());
```
