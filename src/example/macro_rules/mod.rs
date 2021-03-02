// Copyright 2021 Ferris Project Authors. License user Apache License.

// Macro Rules!
// Macros are created using the macro_rules! macro.

// Rust provides a powerful macro system that allows meta-programming.

pub mod say_hello;


/*
So why are macros useful?

1. Don't repeat yourself. There are many cases where you may need similar functionality in multiple
places but with different types. Often, writing a macro is a useful way to avoid repeating code.

2. Domain-specific languages. Macros allow you to define special syntax for a specific purpose.

3. Variadic interfaces. Sometimes you want to define an interface that takes a variable number of
arguments. An example is println! which could take any number of arguments, depending on the format
string!.

 */
