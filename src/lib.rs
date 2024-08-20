//! Macro for static assert that types implement a trait or not.
//!
//! Note: this macro can only be used inside function body due to
//! restriction of Rust.
//!
//! # Example
//!
//! Assuming you have the following definitions:
//! ```
//! struct C;
//! struct Java;
//! struct JavaScript;
//! struct Python;
//! struct Rust;
//!
//! trait StaticTyping {}
//! impl StaticTyping for C {}
//! impl StaticTyping for Java {}
//! impl StaticTyping for Rust {}
//! ```
//!
//! This should build:
//! ```
//! # #[macro_use] extern crate assert_impl;
//! # struct C;
//! # struct Java;
//! # struct JavaScript;
//! # struct Python;
//! # struct Rust;
//! # trait StaticTyping {}
//! # impl StaticTyping for C {}
//! # impl StaticTyping for Java {}
//! # impl StaticTyping for Rust {}
//! assert_impl!(StaticTyping: C, Java, Rust);
//! assert_impl!(StaticTyping: C, Java, Rust, );
//! assert_impl!(!StaticTyping: JavaScript, Python);
//! assert_impl!(!StaticTyping: JavaScript, Python, );
//! ```
//!
//! But these should fail to build:
//! ```compile_fail
//! # #[macro_use] extern crate assert_impl;
//! # struct C;
//! # struct Java;
//! # struct JavaScript;
//! # struct Python;
//! # struct Rust;
//! # trait StaticTyping {}
//! # impl StaticTyping for C {}
//! # impl StaticTyping for Java {}
//! # impl StaticTyping for Rust {}
//! assert_impl!(StaticTyping: JavaScript);
//! ```
//!
//! ```compile_fail
//! # #[macro_use] extern crate assert_impl;
//! # struct C;
//! # struct Java;
//! # struct JavaScript;
//! # struct Python;
//! # struct Rust;
//! # trait StaticTyping {}
//! # impl StaticTyping for C {}
//! # impl StaticTyping for Java {}
//! # impl StaticTyping for Rust {}
//! assert_impl!(!StaticTyping: Rust);
//! ```
#![no_std]

#[macro_export]
macro_rules! assert_impl {
    ($trait:path: $($ty:ty),+) => {
        const _: () = {
            const fn helper<T: $trait>() {}
            $(
                const _: () = helper::<$ty>();
            )+
        };
    };
    (!$trait:path: $($ty:ty),+) => {
        const _: () = {
            #[allow(dead_code)]
            #[allow(clippy::items_after_statements)]
            fn helper_fn() {
                struct Helper<T>(T);
                trait AssertImpl {
                    fn assert() {}
                }
                impl<T: $trait> AssertImpl for Helper<T> {}
                trait AssertNotImpl {
                    fn assert() {}
                }
                $(
                    impl AssertNotImpl for Helper<$ty> {}
                    Helper::<$ty>::assert();
                )+
            }
        };
    };
    ($trait:path: $($ty:ty,)+) => (assert_impl!($trait: $($ty),+));
    (!$trait:path: $($ty:ty,)+) => (assert_impl!(!$trait: $($ty),+));
}

assert_impl!(Sync: u8, usize);
assert_impl!(!Sync: *mut u8, *const usize);
