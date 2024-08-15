//! Lock environment variables to prevent simultaneous access. Use the
//! [EnvGuard] to set values for whatever environment variables you intend to
//! access in your test. This will return a guard that, when dropped, will
//! revert the environment to its initial state. The guard uses a
//! [Mutex] underneath to ensure that multiple tests within the same process
//! can't access it at the same time.
//!
//! > Note: There is a single mutex per process that locks the *entire*
//! environment. This means any code locked by `EnvGuard` cannot run
//! concurrently, even if they don't modify any of the same environment
//! variables. Keep your critical sections as short as possible to prevent
//! slowdowns.
//!
//! ```
//! use env_guard::EnvGuard;
//! use std::env;
//!
//! let var = "ENV_GUARD_TEST_VARIABLE";
//! assert!(env::var(var).is_err());
//!
//! let guard = EnvGuard::lock([(var, Some("hello!"))]);
//! assert_eq!(env::var(var).unwrap(), "hello!");
//! drop(guard);
//!
//! assert!(env::var(var).is_err());
//! ```

#![forbid(unsafe_code)]
#![deny(clippy::all)]

use std::{
    env,
    sync::{Mutex, MutexGuard},
};

/// A guard used to indicate that the current process environment is locked.
/// This should be used in all tests that access environment variables, to
/// prevent interference from external variable settings or tests conflicting
/// with each other.
pub struct EnvGuard<'a> {
    previous_values: Vec<(&'a str, Option<String>)>,
    #[allow(unused)]
    guard: MutexGuard<'static, ()>,
}

impl<'a> EnvGuard<'a> {
    /// Lock the environment and set each given variable to its corresponding
    /// value. The returned guard will keep the environment locked so the
    /// calling test has exclusive access to it. Upon being dropped, the old
    /// environment values will be restored and then the environment will be
    /// unlocked.
    pub fn lock(
        variables: impl IntoIterator<Item = (&'a str, Option<impl AsRef<str>>)>,
    ) -> Self {
        /// Global mutex for accessing environment variables. Technically we
        /// could break this out into a map with one mutex per variable, but
        /// that adds a ton of complexity for very little value.
        static MUTEX: Mutex<()> = Mutex::new(());

        let guard = MUTEX.lock().expect("Environment lock is poisoned");
        let previous_values = variables
            .into_iter()
            .map(|(variable, new_value)| {
                let previous_value = env::var(variable).ok();

                if let Some(value) = new_value {
                    env::set_var(variable, value.as_ref());
                } else {
                    env::remove_var(variable);
                }

                (variable, previous_value)
            })
            .collect();

        Self {
            previous_values,
            guard,
        }
    }
}

impl<'a> Drop for EnvGuard<'a> {
    fn drop(&mut self) {
        // Restore each env var
        for (variable, value) in &self.previous_values {
            if let Some(value) = value {
                env::set_var(variable, value);
            } else {
                env::remove_var(variable);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // NOTE: Because these tests specifically modify environment variables
    // *outside* the env guard, they each need to use a different variable. If
    // only someone make a library that would avoid that...

    /// Set a value for a variable that doesn't exist yet
    #[test]
    fn set_missing_var() {
        let var = "ENV_GUARD_TEST_VARIABLE_SET_MISSING";
        assert!(env::var(var).is_err());

        let guard = EnvGuard::lock([(var, Some("hello!"))]);
        assert_eq!(env::var(var).unwrap(), "hello!");
        drop(guard);

        assert!(env::var(var).is_err());
    }

    /// Override the value for a preexisting variable
    #[test]
    fn set_existing_var() {
        let var = "ENV_GUARD_TEST_VARIABLE_SET_EXISTING";
        env::set_var(var, "existing");
        assert_eq!(env::var(var).unwrap(), "existing");

        let guard = EnvGuard::lock([(var, Some("hello!"))]);
        assert_eq!(env::var(var).unwrap(), "hello!");
        drop(guard);

        assert_eq!(env::var(var).unwrap(), "existing");
    }

    /// Remove the value for a preexisting variable
    #[test]
    fn clear_existing_var() {
        let var = "ENV_GUARD_TEST_VARIABLE_CLEAR_EXISTING";
        env::set_var(var, "existing");
        assert_eq!(env::var(var).unwrap(), "existing");

        let guard = EnvGuard::lock([(var, None::<&str>)]);
        assert!(env::var(var).is_err());
        drop(guard);

        assert_eq!(env::var(var).unwrap(), "existing");
    }
}
