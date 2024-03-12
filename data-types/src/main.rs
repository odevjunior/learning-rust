fn main() {
    operations();
    characterTpe();
}

fn integers() {
    let u: u32 = 10;
    println!("The value of u is {u}");
}

fn floating_numbers() {
    let x = 2.0; //f64 default has double precision
    let y: f32 = 3.0; // f32
}

fn operations() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!("the value of sum {sum}");
    println!("the value of difference {difference}");
    println!("the value of product {product}");
    println!("the value of quotient {quotient}");
    println!("the value of truncated {truncated}");
    println!("the value of remainder {remainder}");
}


fn characterTpe() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{c}");
    println!("{z}");
    println!("{heart_eyed_cat}");
}

fn tuples() {
    let tup(i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    let f = tup.1;
    println!("The value of y is: {y}");
    println!("The value of f is: {f}");
}

fn array() {
    let a =[1,2,3,4,5,6,7,8]; //arrays podem ter somente 1 tipo.    
}