fn main() {

    // let box_demo = Box::new(1);

}


struct Box<T: ?Sized>{
    val: T,
}

impl<T: Clone> Clone for Box<[T]> {
    fn clone(&self) -> Self {
        let mut new = BoxBuilder {
            data: RawVec::with_capacity(self.len());
            len: 0,
        };

        let mut target = new.data.ptr();

        for item in self.iter() {
            unsafe {
                ptr::write(target, item.clone());
                target = target.offset(1);
            };
            new.len += 1;
        }

        return unsafe {
            new.into_box()
        };

        struct BoxBuilder<T> {
            data: RawVec<T>,
            len: usize,
        }

        impl<T> BoxBuilder<T> {
            unsafe fn into_box<self> -> Box<T> {
                let raw = ptr::read(&self.data);
                mem::forget(self);
                raw.into_box()
            }
        }

        impl<T> Drop for BoxBuilder<T> {
            fn drop(&mut self) {
                let mut data = self.data.ptr();
                let max = unsafe {
                    data.offset(self.len as isize) 
                };
                
                while data != max {
                    unsafe {

                        ptr::read(data);
                        data = data.offset(1);
                    }
                }
            }
        }
    }
}