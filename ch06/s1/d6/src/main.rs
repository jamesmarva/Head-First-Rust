fn main() {
    
    let arr = [1i32, 2, 3, 4, 5];
    let ref_arr = &arr;
    println!("address: {:p}", ref_arr);
    println!("size of array: {}", std::mem::size_of::<[i32; 10]>());
    println!("size of ref of array:{}", std::mem::size_of::<&[i32; 5]>());
    view_slice_data(ref_arr);
    // raw_slice(ref_arr);

    // let arr1 = [1i32, 0, 0, 0, 0, 2, 3, 4, 5];
    // println!("address of arr1: {:p}", &arr1);
    // raw_slice(&arr1);
    
}


/// 
/// slice: 
fn raw_slice(arr: &[i32]) {
    unsafe {
        // 
        // let (val1, val2): (usize, usize) = std::mem::transmute::<&[i32], (usize, usize)>(arr);
        let (val1, val2): (usize, usize) = std::mem::transmute(arr);
        println!("Value in raw pointer");
        println!("val1: {:x}", val1);
        println!("val2: {:x}", val2);
        // println!("size of arr: {}", std::mem::size_of_val(arr));
        println!("size of Type: &[i32] : {}", std::mem::size_of::<&[i32]>());
    }
}


fn view_slice_data(sli: &[i32]) {
    unsafe {
        let (v1, v2) : (usize, usize) = std::mem::transmute(sli);
        println!("sli address: {:x}", v1);
        println!("size of array: {:x}", v2);
    }
    unsafe {
        let (v1, v2) : (u64, u64) = std::mem::transmute(sli);
        println!("sli address: {:x}", v1);
        println!("size of array: {:x}", v2);
    }
}

/// slice 是一个特殊的借用
fn size_of_slice() {
    println!("&[i32]: {}", std::mem::size_of::<&[i32]>());
    println!("&[i64]: {}", std::mem::size_of::<&[i64]>());
}


fn size_of_reference_primitive_type() {

}

fn size_of_reference_struct_type() {

}

fn size_of_array() {

}
