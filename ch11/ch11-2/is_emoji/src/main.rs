use std::io::{self, Write};

trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

// Implement IsEmoji for the built-in character type.
impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        true
    }
}

fn main() {
    assert_eq!('$'.is_emoji(), true);
}
