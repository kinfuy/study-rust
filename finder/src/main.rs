use std::process;
fn get_open_path() -> String {
    let mut arg = String::new();
    std::io::stdin().read_line(&mut arg).unwrap();
    return arg;
}
fn main() {
    println!("输入打开的项目?");
    let arg = get_open_path();
    println!("Searching for {}", arg);
    let output = process::Command::new("ls").spawn().expect("执行错误");
    println!("{:?}", output)
}
