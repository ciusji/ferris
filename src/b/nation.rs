pub mod nation {

    pub mod government {
        pub fn govern() {
            println!("nation -> government => govern");
        }
    }

    mod congress {
        pub fn legislate() {}
    }

    mod court {
        fn judicial() {
            super::congress::legislate();
        }
    }
}