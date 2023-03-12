use gacm::load;
fn main() {
    println!("请输入一个config 地址");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    println!("Searching for {}", input);

    let path =  String::from("export const HOME = process.env[process.platform === 'win32' ? 'USERPROFILE' : 'HOME'] || '';");

    load::set_config(&input, &path);

    let file = load::config(&input);

    println!("{}", file);
}
