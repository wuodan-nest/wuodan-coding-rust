fn main() {
    let rect = Rectangle {
	width: 10,
	height: 20,
    }

    println!(
        "The area of the rectangle is {:#?} square pixels.",
        area(rect)
    );
}

struct Rectangle {
    width: i32,
    height: i32,
}

fn area(rectangle: &Rectangle) -> i32 {
    width * height
}
