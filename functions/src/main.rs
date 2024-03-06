use rand::Rng;

fn main() {
    println!("Hello, world!");

    another_function(32);
    expressions();
    println!("return function {}", return_function());
}

fn another_function(x: i32) {
    println!("Another function. {x}")
}


fn expressions() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn return_function() -> u32 {
    return rand::thread_rng().gen_range(1..=100);
}