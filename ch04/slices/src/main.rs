fn main() {
    // need to slice only at valid positions for multibyte characters
    let mut s = String::from("hello world");

    let hello = &s[..5]; //same as [0..5];
    let world = &s[6..]; //same as [6..11];

    let firstword = first_word(&s);
    println!("{},{},{}", hello, world, firstword);

    s = String::from("hi"); // can't use hello, world, or firstword after this since they are immutable borrows
    println!("{}", s);

    let mut t = "Hello, World!";
    println!("{}", t);
    t = "asdf";
    println!("{}", t);

    let a = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    let a_s = &a[3..6]; // type &[i32]
}

// change input type in signature to be a slice instead of the specific type of variable
// passing &String will be automatically cast to a &str
//fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
