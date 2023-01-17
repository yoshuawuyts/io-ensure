//! Prototype of the `std::io::ensure` family of macros

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, unreachable_pub)]

/// Creates an [`io::Error`] using optional interpolation of runtime expressions.
///
/// Arguments to `format_err!` can either be literals which are passed to
/// `io::Error::new`, or directly interpolated strings constructed through the
/// `format!` macro.
///
/// See [`std::fmt`] for more information.
///
/// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
/// [`std::fmt`]: ../std/fmt/index.html
/// [`print!`]: ../std/macro.print.html
/// [`write!`]: core::write
/// [`to_string`]: crate::string::ToString
/// [`Display`]: core::fmt::Display
///
/// # Panics
///
/// `format_err!` panics if a formatting trait implementation returns an error.
/// This indicates an incorrect implementation since `fmt::Write for String`
/// never returns an error itself.
///
/// # Examples
///
/// ```
/// use io_ensure::format_err;
/// use std::io::ErrorKind;
///
/// // errors can be created from format strings
/// let custom_error = format_err!(ErrorKind::Other, "hello {}", "world!");
///
/// // errors can also be created from other errors
/// let custom_error2 = format_err!(ErrorKind::Interrupted, custom_error);
///
/// // errors can also be created without payload (and without memory allocation)
/// let eof_error = format_err!(ErrorKind::UnexpectedEof);
/// ```
#[macro_export]
macro_rules! format_err {
    ($kind:expr, $msg:literal $(,)?) => {{
        ::std::io::Error::new($kind, $msg)
    }};
    ($kind:expr, $msg:expr $(,)?) => {{
        ::std::io::Error::new($kind, $msg)
    }};
    ($kind:expr, $msg:expr, $($arg:tt)*) => {{
        ::std::io::Error::new($kind, format!($msg, $($arg)*))
    }};
    ($kind:expr $(,)?) => {{
        ::std::io::Error::from($kind)
    }};
}

/// Exits a function early with an [`io::Error`] if the condition is not satisfied.
///
/// Similar to [`assert!`], `ensure!` takes a condition and exits the function
/// if the condition fails. Unlike `assert!`, `ensure!` returns an `io::Error`,
/// it does not panic.
///
/// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
/// [`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
/// [`assert!`]: https://doc.rust-lang.org/std/macro.assert.html
///
/// # Examples
///
/// ```
/// # use io_ensure::*;
/// use std::io::ErrorKind;
///
/// # fn main() -> std::io::Result<()> {
/// let a = 3;
/// let b = 1 + 2;
/// ensure!(a == b, ErrorKind::Other);
///
/// ensure!(a == b, ErrorKind::Interrupted, "we are testing addition with {} and {}", a, b);
/// # Ok(()) }
/// ```
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $kind:expr, $msg:literal $(,)?) => {
        if !$cond {
            return ::std::result::Result::Err($crate::format_err!($kind, $msg));
        }
    };
    ($cond:expr, $kind:expr, $msg:expr $(,)?) => {
        if !$cond {
            return ::std::result::Result::Err($crate::format_err!($kind, $msg));
        }
    };
    ($cond:expr, $kind:expr $(,)?) => {
        if !$cond {
            return ::std::result::Result::Err($crate::format_err!($kind));
        }
    };
    ($cond:expr, $kind:expr, $msg:expr, $($arg:tt)*) => {
        if !$cond {
            return ::std::result::Result::Err(format_err!($kind, $msg, $($arg)*));
        }
    };
}

/// Exits a function early with an [`io::Error`] if two expressions are not equal
/// to each other.
///
/// The comparison is performed using [`PartialEq`]. Similar to [`assert_eq!`],
/// `ensure_eq!` takes two expressions and exits the function if the comparison
/// fails. Unlike `assert_eq!`, `ensure!` returns an `io::Error`, it does not panic.
///
/// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
/// [`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
/// [`assert_eq!`]: https://doc.rust-lang.org/std/macro.assert_eq.html
///
/// # Examples
///
/// ```
/// # use io_ensure::*;
/// use std::io::ErrorKind;
///
/// # fn main() -> std::io::Result<()> {
/// let a = 2;
/// let b = 1 + 1;
/// ensure_eq!(a, b, ErrorKind::Other);
///
/// ensure_eq!(a, b, ErrorKind::Interrupted, "we are testing the values {} and {} are equal", a, b);
/// # Ok(()) }
/// ```
#[macro_export]
macro_rules! ensure_eq {
    ($left:expr, $right:expr, $kind:expr, $msg:literal $(,)?) => {
        $crate::ensure!($left == $right, $kind, $msg);
    };
    ($left:expr, $right:expr, $kind:expr, $msg:expr $(,)?) => {
        $crate::ensure!($left == $right, $kind, $msg);
    };
    ($left:expr, $right:expr, $kind:expr, $msg:expr, $($arg:tt)*) => {
        $crate::ensure!($left == $right, $kind, $msg, $($arg)*);
    };
    ($left:expr, $right:expr, $kind:expr $(,)?) => {
        $crate::ensure!($left == $right, $kind);
    };
}

/// Exits a function early with an [`io::Error`] if two expressions are equal to
/// each other.
///
/// The comparison is performed using [`PartialEq`]. Similar to [`assert_eq!`],
/// `ensure_eq!` takes two expressions and exits the function if the comparison
/// fails. Unlike `assert_eq!`, `ensure!` returns an `io::Error`, it does not panic.
///
/// [`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
/// [`PartialEq`]: https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
/// [`assert_ne!`]: https://doc.rust-lang.org/std/macro.assert_ne.html
///
/// # Examples
///
/// ```
/// # use io_ensure::*;
/// use std::io::ErrorKind;
///
/// # fn main() -> std::io::Result<()> {
/// let a = 2;
/// let b = 3;
/// ensure_ne!(a, b, ErrorKind::Other);
///
/// ensure_ne!(a, b, ErrorKind::Interrupted, "we are testing the values {} and {} are not equal", a, b);
/// # Ok(()) }
/// ```
#[macro_export]
macro_rules! ensure_ne {
    ($left:expr, $right:expr, $kind:expr, $msg:literal $(,)?) => {
        $crate::ensure!($left != $right, $kind, $msg);
    };
    ($left:expr, $right:expr, $kind:expr, $msg:expr $(,)?) => {
        $crate::ensure!($left != $right, $kind, $msg);
    };
    ($left:expr, $right:expr, $kind:expr, $msg:expr, $($arg:tt)*) => {
        $crate::ensure!($left != $right, $kind, $msg, $($arg)*);
    };
    ($left:expr, $right:expr, $kind:expr $(,)?) => {
        $crate::ensure!($left != $right, $kind);
    };
}
