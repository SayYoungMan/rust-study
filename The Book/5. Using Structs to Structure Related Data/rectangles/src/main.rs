fn main() {
    let width = 30;
    let height = 50;
    print_area(area_first(width, height));

    let rect1 = (30, 50);
    print_area(area_tuple(rect1));

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect is {:#?}", rect);
    dbg!(&rect);
    print_area(area(&rect));
}

fn print_area(area: u32) {
    println!("The area of the rectangle is {} square pixels.", area);
}

// First version that takes two arguments
fn area_first(width: u32, height: u32) -> u32 {
    width * height
}

// Tuple adds some structure and pass one argument
// But it's less clear as tuples don't name elements
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// struct version of area function
// Have reference as argument so main retains ownership and can continue using rect
fn area(rectangle: &Rectangle) -> u32 {
    // Accessing fields of borrowed struct instance does not move the field values
    rectangle.width * rectangle.height
}
