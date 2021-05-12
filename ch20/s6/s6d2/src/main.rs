fn main() {
    let mut ori = vec![1i32, 2, 3, 4];

    println!("remove");
    // drain [)
    for i in ori.drain(1..3) {
        println!("{}", i);
    }

    println!("left");
    for i in ori.iter() {
        println!("{}", i);
    }

    for i in ori {
        println!("{}", i)
    }
}


pub fn drain<R>(&mut self, range: R) -> Drain<T>
where R: RangeArgument<usize> 
{
    let len = self.len();
    let start = match range.start() {
        Included(&n) => n,
        Excluded(&n) => n + 1,
        Unbounded => 0,
    };

    let end = match range.end() {
        Included(&n) => n + 1,
        Excluded(&n) => n,
        Unbounded => len,
    };

    assert!(start <= end);
    assert!(end <= len);

    unsafe {
        self.set_len(start);
        let range_slice = slice::from_raw_parts_mut(self.as_mut_ptr().offset(start));

        Drain {
            tail_start: end,
            tail_len: len -end,
            iter: range_slice.iter(),
            vec: Shared::from(self),
        }
    }
}


