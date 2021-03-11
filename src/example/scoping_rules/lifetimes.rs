//! Copyright 2021 Ferris Project Authors. License user Apache License.

// A lifetime is a construct the compiler (or more specifically, it borrow checker) uses to ensure
// all borrows are valid. Specially, a variable's lifetime begins when it is created and ends when it
// is destroyed. While lifetimes and scopes are often referred to together, they are not the same.