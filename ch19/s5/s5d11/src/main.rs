use std::fmt::Debug;
use std::ptr::null;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
struct S;


#[derive(Debug)]
struct R<'a, T: Debug + 'a> {
    x: *const T,
    marker: PhantomData<&'a T>,
}

impl<'a, T: Debug> Drop for R<'a, T> {
    fn drop(&mut self) {
        unsafe {
            println!("Dropping R while S {:?}", *self.x);
        }
    }
}

impl <'a, T: Debug + 'a> R<'a, T> {
    pub fn ref_to<'b: 'a>(&mut self, obj: &'b T) {
        self.x = obj;
    }
}

fn main() {
    let mut r = R {x: null(), marker: PhantomData};

    {
        let local = S {};
        //，这次编译器就可以成功发现生命周期错误，禁止悬挂指针的产生。
        // r.ref_to(&local);
    }

    let local = S {};
    r.ref_to(&local);


}






