use std::ops::Add;

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    let p1 = Millimeters(8);
    let p2 = Meters(10);
    let p3 = p1 + p2;
    println!("{:?}", p3)
}
