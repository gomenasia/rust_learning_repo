fn main() {

    let hello = String::from("Hello World!");
    let rep = first_word(&hello);

    println!("{}", rep)

    let mut five = another_function
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
    5
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}