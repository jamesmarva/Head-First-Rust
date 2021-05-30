fn main() {
    let mut a = [1u32; 101];
    let mut_ref_a = &mut a;
    mut_slice_0(mut_ref_a);
    mut_slice_100(mut_ref_a);
}


/// slice
fn mut_slice_100(a: &mut [u32]) {
    a[100] = 100;
}

fn mut_slice_0(a: &mut [u32]) {
    a[0] = 1000;
}
