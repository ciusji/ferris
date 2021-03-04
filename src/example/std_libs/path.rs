// Copyright 2021 Ferris Project Authors. License user Apache License.

// Path
// The Path struct represents file paths in the underlying filesystem. The are two flavors of Path:
// - posix::Path, for UNIX-like systems.
// - windows::Path for windows.

use std::path::Path;

pub fn path_info() {
    // Create a `Path` from an `&'static str`
    let path = Path::new(".");

    // The `display` method returns a `Show` structure
    let _display = path.display();

    println!(".. {:?}", path.to_str());
}