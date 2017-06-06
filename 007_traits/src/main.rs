
// Defines a trait for something that has an area
trait HasArea {
    fn area(&self) -> f64;
}

// Some structs

struct Circle {
    radius: f64,
}

struct Rectangle<T> {
    width: T,
    height: T,
}

// implement the trait for above struct
impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

/* FAILS to compile:
 * error[E0277]: the trait bound `f64: std::convert::From<<T as std::ops::Mul>::Output>` is not satisfied
 *  --> src\main.rs:27:9
 *   |
 *27 |         f64::from(self.width * self.height)
 *   |         ^^^^^^^^^ the trait `std::convert::From<<T as std::ops::Mul>::Output>` is not implemented for `f64`
 *   |

impl<T: std::ops::Mul + From<f64>> HasArea for Rectangle<T> {
    fn area(&self) -> f64 {
        f64::from(self.width * self.height)
    }
}
*/

// Traits implementation below compile, but they are not generic

impl HasArea for Rectangle<i32> {
    fn area(&self) -> f64 {
        f64::from(self.width * self.height)
    }
}

impl HasArea for Rectangle<f32> {
    fn area(&self) -> f64 {
        f64::from(self.width * self.height)
    }
}



fn main() {
    let circle1 = Circle{radius: 1.0};
    println!( "circle1 area={}", circle1.area() );
    
    let int_rect = Rectangle{width: 1, height: 1};
    println!( "int_rect area={}", int_rect.area() );

    let f32_rect = Rectangle{width: 1f32, height: 1f32};
    println!( "f32_rect area={}", f32_rect.area() );
}
