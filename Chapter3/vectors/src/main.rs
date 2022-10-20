fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// either an array or a vector can be a slice
fn add_one(a: &mut [i32]) {
    for i in 0..a.len() {
        a[i] += 1;
    }
}

fn arrays() {
    let a = [0; 6];               // array, size known at compile time
    print!("a is "); print_type_of(&a);

    let mut b = [1,2,3,4,5,6];    // array, again with known size
    print!("b is "); print_type_of(&b);

    let c = &mut b;               // mutable reference to array
    print!("c is "); print_type_of(&c);
    c[1] = 10;

    println!("{:?}", b);  // change in c[] changes b 
    
    let d = b;                    // ok because i32 has COPY type
    print!("d is "); print_type_of(&d);
    b[2] = 20;

    let e = b.clone();
    print!("e is "); print_type_of(&e);
    b[3] = 30;

    println!("{:?} {:?} {:?}", b, d, e);  // changes in b after copy or clone not seen 

    let f = &b as *const i32;     // raw pointer
    print!("f is "); print_type_of(&f);

    println!("{:?}", f);          // prints the address
    for i in 0..b.len() {
        unsafe{ println!("{} {:?}", i, *(f.add(i))); }
    }

    let g = &mut b[3..5];      // mutable slice of an array
    print!("g is "); print_type_of(&g);
    println!("{:?}", g);
    g[1] = 40;

    println!("{:?}", b);   // changes to g[] reflected in b

    add_one(&mut b);   // ok to call with array slice
    println!("{:?}", b);

    println!();
}

fn vectors() {
    let a_size = 6;
    let a = vec![0; a_size];      // vector, size determined at runtime
    print!("a is "); print_type_of(&a);

    let mut b : Vec<_> = (1..=5).collect();   // vector can be dynamic
    print!("b is "); print_type_of(&b);
    b.push(6);
 
    let c = &mut b;
    print!("c is "); print_type_of(&c);
    c[1] = 10;

    println!("{:?}", b);  // change in c changes b 

    let mut d = b;                // big difference between arrays and vectors
                                  // this is a change in ownership not a copy.
                                  // Vec<i32> does not implement the Copy trait
    print!("d is "); print_type_of(&d);
    // b[2] = 20;     // "value borrowed here after move"
    d[2] = 20;

    let mut e = d.clone();
    print!("e is "); print_type_of(&e);
    d[3] = 30;
    e[3] = 40;

    println!("{:?} {:?}", d, e);  // d and e are separate now
    
    let f = d.as_ptr();      // Since Vec contains a header with length and
                             // capacity, you cannot just cast.  Instead, 
                             // use .as_ptr() or .as_mut_ptr()
    print!("f is "); print_type_of(&f);

    println!("{:?}", f);      // prints the address (note difference with
                              // array address, heap vs stack)
    for i in 0..d.len() {
        unsafe{ println!("{} {:?}", i, *(f.add(i))); }
    }


    let g = &mut d[3..5];      // mutable slice of a vector
    print!("g is "); print_type_of(&g);
    println!("{:?}", g);
    g[1] = 40;

    println!("{:?}", d);   // changes to g[] reflected in b

    add_one(&mut d);   // ok to call with vector slice
    println!("{:?}", d);
    
    println!();
}


fn main() {
    arrays();
    vectors();
}