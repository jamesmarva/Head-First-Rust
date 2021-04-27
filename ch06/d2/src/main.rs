fn main() {

    let mut arr1 = [1, 3, 4, 4, 5];

    println!("arr1: {:?}", arr1);

    let arr2 = [2, 2, 2, 2, 2, 2];

    println!("arr2: {:?}", arr2);

    // 
    let arr3: [i32; 5] = [1, 1, 1, 1, 1];

    // 同类型的数组才能赋值，但是什么是同类型？元素的类型相同，数组能够存放的元素的个数也是相同。
    // arr1 = arr2;

    // 元素类型、个数都相同
    arr1 = arr3;
    println!("arr1: {:?}", arr1);
}
