fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        // 長方形の面積は、{}平方ピクセルです
        "The area of the rectangle is {} square pixels.",
        area((width1, height1));
    );
}

fn area(dimensions:( u32, u32 )) -> u32 {
    dimensions.0 * dimensions.0
}
