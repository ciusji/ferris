// Copyright 2021 Ferris Project Authors. License user Apache License.

// The super and self keywords can be used in the path to remove ambiguity when accessing items
// and to prevent unnecessary hardcoding of paths.

fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_all() {
        // Let's access all the functions named `function` from scope.
        println!("called `my::indirect_all()`, that\n> ");

        // The `self` keyword refers to the current module scope - in this case `my`.
        self::function();
        function();

        // We can also use `self` to access another module inside `my`.
        self::cool::function();

        // The `super` keyword refers to the parent scope (outside the `my` module).
        super::function();

        // This will bind the `cool::function` in the *crate* scope.
        // In this case the crate scope is the outermost scope.
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

pub fn super_self() {
    my::indirect_all();
}