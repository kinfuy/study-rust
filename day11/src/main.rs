struct Cacher<T, U>
where
    T: Fn(U) -> U,
{
    query: T,
    value: Option<U>,
}

impl<T, U> Cacher<T, U>
where
    U: Copy,
    T: Fn(U) -> U,
{
    fn new(query: T) -> Cacher<T, U> {
        Cacher { query, value: None }
    }

    // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
    fn value(&mut self, arg: U) -> U {
        match &self.value {
            Some(v) => *v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let mut ca = Cacher::new(|x| x);
    println!("{:?}", ca.value("33"));
    println!("{:?}", ca.value("13"));
    ca.value = Some("44");
    println!("{:?}", ca.value("13"));
    println!("{:?}", ca.value("13"));
    ca.value = Some("77");
    println!("{:?}", ca.value("13"));
}
