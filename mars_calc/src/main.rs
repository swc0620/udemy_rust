fn main() {
    // exclamation mark is a macro not a function call
    // macro can receive various number of parameters with different types
    println!("Hello, world!");

    // Here, we can omit type declaration because Rust compiler infers data type from return type of the function 
    // explicitly put mut keyword to make variable mutable
    let mut mars_weight = calculate_weight_on_mars(100.0);
    mars_weight = mars_weight * 1000.0;
    println!("Weight on Mars: {}kg", mars_weight);

    calculate_weight_on_mars(100.0);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    // last expression without semi-colon is equal to return value
    (weight / 9.81) * 3.711
}