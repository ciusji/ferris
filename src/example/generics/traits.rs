//! Copyright 2021 Ferris Project Authors. License user Apache License.

// Non-copyable types.
struct Empty;
struct Null;

// A trait generic over `T`
trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

// Implement `DoubleDrop<T>` for any generic parameter `T` and caller `U`
impl<T, U> DoubleDrop<T> for U {
    // This method takes ownership of both passed arguments, deallocating both.
    fn double_drop(self, _: T) {}
}


pub fn traits() {
    let empty = Empty;
    let null = Null;

    // Deallocate `empty` and `null`.
    empty.double_drop(null);

    // empty;
    // null;
}