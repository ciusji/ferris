//! Copyright 2021 Ferris Project Authors. License user Apache License.

// Lifetime - Functions

// Ignore elision, function signatures with lifetimes have a few constraints:
// - any reference must have an annotated lifetime.
// - any reference being returned must have the same lifetime as an input or be static.
//
// Additionally, note that returning references without input is banned if it would result in
// returning references to invalid data.