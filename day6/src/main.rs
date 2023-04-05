use std::ops::Add;

#[derive(Debug)]
struct Point<T>
where
    T: Add<T, Output = T>,
{
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Point<T> {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1.8, y: 1.9 };
    let p2 = Point { x: 1.4, y: 3.33 };

    let p3 = p1 + p2;

    println!("{:#?}", p3);
}
