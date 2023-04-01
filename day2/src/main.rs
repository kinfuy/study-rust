fn main() {
    fn add(a: &u32, b: u32) -> u32 {
        a + b
    }
    let mut rst = 0;
    loop {
        rst = add(&rst, 3);
        if rst > 100 {
            break;
        }
    }
    print!("rst:{}", rst)
}
