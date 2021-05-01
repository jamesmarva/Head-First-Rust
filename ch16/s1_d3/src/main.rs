use std::ops;
fn main() {
    
}

impl ops::Deref for String {

    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        unsafe {
            str::from_utf8_unchecked(&self.vec) 
        }
    }
}
