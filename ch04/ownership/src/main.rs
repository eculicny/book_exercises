fn main() {
    let s1 = String::from("Test");

    /*
        not a shallow copy of pointer/len/capacity but a _move_
        invalidates referencing s1 after the assignment to s2
    */
    let s2 = s1;

    //println!("{} string", s1); // compile-time error due to move

    /* Copying heap data into new var */
    let s3 = String::from("hello");
    let s4 = s3.clone(); // copies heap data and creates new pointer for s4

    println!("s3 = {}, s4 = {}", s3, s4);

    /* Copying stack data in to new var */
    let x = 5;
    let mut y = x;
    println!("x={}, y={}", x, y); // stack vars automatically copy instead of moving

    y = 6;
    println!("x={}, y={}", x, y);
    // The "Copy" trait annotation allows automatic copying instead of moving and invalidating previous vars
    // The "Drop" trait annotation prevents use of the "Copy" trait and the type will exhibit the heap behavior

    /* tuples split copy properties, tuple as a whole is not copy*/
    let (i, s) = (123, String::from("asdf"));
    let (mut ii, ss) = (i, s);

    ii = 7890;
    println!("i = {}, ii = {}, s = {}", i, ii, ss); // can't reference s
}
