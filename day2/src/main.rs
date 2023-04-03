// fn main() {
//     // let a = 1;
//     // let b: u32 = 2;
//     // let c = true;
//     // let d = 'c';
//     // let e = "cc";

//     // const TEST: &str = "TEST_CONST";

//     // let arr = [3; 5];
//     // // let arr = [3; 10];

//     // fn test(str: &str) {
//     //     println!("test:{}", &str);
//     // }
//     // test(TEST);

//     // println!("{}", TEST);

//     // println!("{arr:?}");

//     // println!("{}", arr[1]);

//     // let tup = (1, 2, 3);

//     // println!("{}", tup.0);

//     // println!("{arr:?}");

//     // println!("{tup:?}");
//     // print!("{},{},{},{},{}", a, b, c, d, e);

//     // let condition = true;
//     // let number = if condition { 5 } else { 6 };

//     // println!("The value of number is: {number}");

//     // let mut count = 0;
//     // 'test: loop {
//     //     println!("count = {count}");
//     //     let mut remaining = 10;

//     //     loop {
//     //         println!("remaining = {remaining}");
//     //         if remaining == 9 {
//     //             break;
//     //         }
//     //         if count == 2 {
//     //             break 'test;
//     //         }
//     //         remaining -= 1;
//     //     }

//     //     count += 1;
//     // }
//     // println!("End count = {count}");

//     let s1 = String::from("hello");
//     let s2 = s1.clone();

//     println!("s1 = {}, s2 = {}", s1, s2);

//     // let mut s = String::from("Hello");
//     // s.push_str(", World!");
//     // println!("{}", s);
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word is: {}", word);

    s.clear(); // 错误！
}
