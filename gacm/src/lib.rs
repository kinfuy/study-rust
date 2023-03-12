pub mod load {

    use std::fs;

    pub fn config(path: &String) -> String {
        let file = fs::read_to_string(path.trim()).expect("file not found");
        file
    }

    pub fn set_config(path: &String, content: &String) {
        fs::write(path.trim(), content).expect("发生了错误");
        print!("我需要打开配置文件,{}", path);
    }
}
