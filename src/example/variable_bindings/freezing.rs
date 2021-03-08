//! Copyright 2021 Ferris Project Authors. License user Apache License.

// Variable Binding - Freezing
// When data is bound by the same name immutably, it also freezes. Frozen data can't be modify until
// the immutable binding goes out of scope.

pub fn freezing() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer`
        let _mutalbe_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        _mutable_integer = 50;

    } // `_mutable_integer` goes out of scope

    // `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;

}