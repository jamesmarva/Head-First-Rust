fn main() {
    
}

fn compare_option<T>(first: Option<T>, second: Option<T>) -> bool {
    match(first, second) {
        (Some(..), Some(..)) => true,
        (None, None)  => true,
        _ => false,
    
    }
}
