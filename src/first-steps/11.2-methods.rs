// Methods are functions attached to objects.
// Those methods have the access to the contents of the object and the other methods via the
// 'self' keyword. Methods are defined in the 'impl' block

struct Point {
    x: f64,
    y: f64
}


// Object with two static methods only
impl Point {

    fn origin() -> Point {
        Point { x: 0f64, y:0f64 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point {x, y}
    }
}

struct Rectangle {
    p1: Point,
    p2: Point
}

impl Rectangle {

    fn area(&self) -> f64 {
        let Point { x:x1, y:y1} = self.p1;
        let Point {x:x2, y: y2} = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    // '&self' is a syntax sugar for the '&self: Self' where Self is type type of the caller.
    fn perimeter(&self: Self) -> f64 {
        let Point { x:x1, y: y1} = self.p1;
        let Point { x:x2, y: y2} = self.p2;

        2 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    fn translate(&mut self, x:f64, y:f64) {
        let Point { x:x1, y: y1} = self.p1;
        let Point { x:x2, y: y2} = self.p2;

        self.p1 = Point {x: x1 + x, y: y1 + y};
        self.p2 = Point {x: x2 + x, y: y2 + y};
    }
}

fn main() {

}
