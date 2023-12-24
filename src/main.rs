fn main() {
    println!("Hello, max!");

    foobar(16);
}

fn foobar(max_value: i32) {
    println!("max_value: {}", max_value);
    for i in 1..=max_value {
        if i % 3 == 0 {
            if i % 5 == 0 {
                println!("FooBar");
            } else {
                println!("Foo");
            }
        } else if i % 5 == 0 {
            println!("Bar");
        } else {
            println!("{}", i);
        }
    }
}
