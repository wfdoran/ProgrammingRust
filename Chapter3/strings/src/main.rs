fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn string_literals() {
    let a = "hello";
    print!("a is "); print_type_of(&a);
    let b = "hello\n";    // escape characters
    print!("b is "); print_type_of(&b);
    let c = r"hello\n";   // raw string
    print!("c is "); print_type_of(&c);
    let d = b"hello";     // byte string (essential a slice [u8])
    print!("d is "); print_type_of(&d);
    println!("{} {} {} {:?}", a, b, c, d); 

    println!();
}

fn strings() {
    let a = String::from("hello string");      // String 
    print!("a is "); print_type_of(&a);
    let b = &a;                                // reference to a String
    print!("b is "); print_type_of(&b);
    let c = &a[..];                            // slices are string literals
    print!("c is "); print_type_of(&c);
    let d = &a[6..9];
    print!("d is "); print_type_of(&d);
    println!("{} {} {} {}", a, b, c, d);

    let e = "Fix ne!".to_string();         
    let mut f : Vec<_> = e.chars().collect();       // cannot access e[4] directly, but an collect into a vector of chars
    print!("f is "); print_type_of(&f);
    print!("f[0] is "); print_type_of(&f[0]);
    println!("{} {:?}", e, f);

    f[4] = 'm';
    let e : String = f.iter().collect();            // convert back to string, need to tell rust it is a String
    println!("{}", e);

    println!();
}

fn main() {
    string_literals();
    strings();
}
