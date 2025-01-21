fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y += 1; // Modify x through y
    }
    let z = &x; // z is an immutable reference to x

    println!("x = {}", x); // x is now 6
    println!("z = {}", *z); // This will correctly print 6 now because the scope is well-managed
} 