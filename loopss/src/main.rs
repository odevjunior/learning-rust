fn main() {
    range_loop();
}

fn simple_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
        println!("The counter is {counter}");
    };

    println!("The result is {result}");
}

fn looping_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}//usar a aspas simples voce pode nomear um loop e para-lo dentro de outro loop

fn while_loop() {
    let mut number = 3;

    'first_while: while number != 0 {
        println!("{number}!");
        while number > 1 {
            number += 1;
            println!("{number}!");
            if number == 5 {
                break 'first_while;
            }
        }
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn looping_through_collection() {
    let a = [1,2,3,4,5,6,7,8];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

fn for_each() {
    let a = [1,23,2,2,1];

    for mut element in a {
        element += 1;
        println!("the value is: {element}");
    }
}

fn range_loop() {
    for number in (1..5).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}