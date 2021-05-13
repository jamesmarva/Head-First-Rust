fn main() {
    {
        let (a, mut b) : (i32, Option<&i32>) = (1, None);
        b = Some(&a);
    }

    {
        let (mut a, b) : (Option<&i32>, i32) = (None, 1);
        a = Some(&b);
    }
    
}
