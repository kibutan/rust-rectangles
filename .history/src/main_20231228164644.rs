fn main() {
    let rect1=(30,50);

    println!(
        // 長方形の面積は、{}平方ピクセルです
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions:( u32, u32 )) -> u32 {
    dimensions.0 * dimensions.0
}
