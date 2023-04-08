use std::ops::Add;

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl Add for Point<i32, i32> {
    type Output = Point<i32, i32>;
    fn add(self, rhs: Self) -> Point<i32, i32> {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Add for Point<&str, &str> {
    type Output = Point<String, String>;
    fn add(self, rhs: Self) -> Point<String, String> {
        Point {
            x: format!("{}{}", self.x, rhs.x).clone(),
            y: format!("{}{}", self.y, rhs.y).clone(),
        }
    }
}

fn main() {
    let p1 = Point { x: "3", y: "5" };
    let p2 = Point { x: "3", y: "4" };

    let p3 = p1 + p2;

    println!("{:#?}", p3);
}
