fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}", s1, len);

    /////////////////////////////////////////////////
    let mut s = String::from("hello");
    change(&mut s);
    let r = &mut s;
    // let r1 = &mut s; // can't have two mutable references to the same variable
    // can't be borrowed as immutable if it is already borrowed by r as mutable
    println!("{}", s);
    //println!("{}", r); // see above line
    //println!("{},{}", r, r1);

    /////////////////////////////////////////////////
    let mut t = String::from("hello");
    // can just create scope to allow multiple mutable references to t
    {
        let r1 = &mut t;
        println!("{}", r1);
    }
    let r2 = &mut t;
    println!("{}", r2);

    /////////////////////////////////////////////////
    let mut q = String::from("hello");

    let q1 = &q; // no problem
    let q2 = &q; // no problem
    println!("{} and {}", q1, q2);
    // r1 and r2 are no longer used after this point

    let q3 = &mut q; // no problem
    change(q3);
    println!("{}", q3);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// passing reference doesn't change ownership of the variable
fn calculate_length(s: &String) -> usize {
    //s.push_str(", world"); // not possible since the reference is borrowed
    s.len()
}
