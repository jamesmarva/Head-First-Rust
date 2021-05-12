struct T {
    dropped: bool
}

impl T {
    fn new() -> Self {
        T {dropped: false}
    }
}

impl Drop for T {
    fn drop(&mut self) {
        self.dropped = true;
    }
}

struct R<'a> {
    inner: Option<&'a T>
}

impl<'a> R<'a> {
    fn new() -> Self {
        R {
            inner: None
        }
    }

    fn set_ref<'b: 'a>(&mut self, ptr: &'b T) {
        self.inner = Some(ptr);
    }
}

impl<'a> Drop for R<'a> {
    fn drop(&mut self) {
        if let Some(ref inner) = self.inner {
            println!("droppen R when T.dropped is {}", inner.dropped);
        }
    }
}

fn main() {
 
    {
        let (a, mut b) : (T, R) = (T::new(), R::new());
        b.set_ref(&a)
    }

    {
        // 要保证 参数的生命抽要大于等于 'a
        let (mut c, d) : (R, T) = (R::new(), T::new());
        // c.set_ref(&d);
    }

    
}