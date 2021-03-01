// Copyright 2021 Ferris Project Authors. License user Apache License.

// Attributes
// An attribute is metadata applied to some module, crate or item. This metadata can be used to/for:
// - conditional compilation of code
// - set crate name, version and type (binary or library)
// - disable lints (warnings)
// - enable compiler features (macros, glob imports, etc.)
// - link to a foreign library
// - mark functions as unit tests
// - mark functions what will be part of benchmark

pub mod cfg;