//! Copyright 2021 Ferris Project Authors. License user Apache License.

/// Explicit Annotation
/// The borrow checker uses explicit lifetime annotations to determine how long references should be
/// valid. In cases where lifetime are not elided, Rust requires explict annotations to determine what
/// the lifetime of a reference should be. The syntax for explicitly annotating a lifetime uses an
/// apostrophe character as follows:
/// ```rust
/// foo<'a>
/// // `foo` has lifetime parameter `'a`
/// ```
/// Additionally, this lifetime syntax indicates that the lifetime of `foo` may not exceed that of
/// `'a`. Explicit annotation of a type has the form `&'a T` where `'a` has already been introduced.