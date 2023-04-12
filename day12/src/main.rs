fn main() {
    let arr1 = vec![1];
    let arr2 = vec![2, 3];

    let arr3: Vec<_> = arr1.iter().zip(arr2.iter()).collect();
    println!("{:#?}", arr3)
}
