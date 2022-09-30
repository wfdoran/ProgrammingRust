
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


#[derive(Debug)]
struct Point(u32, u32);

#[derive(Debug,Copy,Clone)]
struct Foo(u32, u32);

fn example1() {
    println!("Example1:");
    let a = [1,2,3];

    println!("{:?}", a);
    for i in a {
        println!("{}", i);
    }

    println!("{:?}", a); 
    println!();
}


fn example2() {
    println!("Example2:");
    let a = [Point(1,2), Point(3,4), Point(5,6)];

    println!("{:?}", a);
    for i in a {
        println!("{:?}", i);
    }

    // NB: Point does not implement the Copy and Clone trait.
    // In the for loop, i takes ownership of the elements of a one by one.
    // When the iteration ends, it is dropped.
    // a at this point has been destroyed!, and cannot be printed.

    // println!("{:?}", a);
    println!();
}

fn example3() {
    println!("Example3:");
    let a = [Foo(1,2), Foo(3,4), Foo(5,6)];

    println!("{:?}", a);
    for i in a {
        println!("{:?}", i);
    }

    // Foo does implement Copy and Clone.  So, i in the for loop 
    // gets a cloned copy of each Foo() just like example1.
    println!("{:?}", a);
    println!();
}

fn example4() {
    println!("Example4:");
    let a = [Point(1,2), Point(3,4), Point(5,6)];

    println!("{:?}", a);
    for i in &a {
        println!("{:?}", *i);
    }

    // 
    println!("{:?}", a);
    println!();
}

fn example5() {
    println!("Example5:");
    let a = [Point(1,2), Point(3,4), Point(5,6)];

    println!("{:?}", a);
    for i in 0..a.len() {
        println!("{:?}", a[i]);
    }

    // In this case, println! borrowed each a[i], printed it, and returned it.
    // a is still available here.
    println!("{:?}", a);
    println!();
}

fn example6() {
    println!("Example6:");
    let a = [Point(1,2), Point(3,4), Point(5,6)];

    println!("{:?}", a);
    for i in a.iter() {
        print_type_of(&i);
        println!("{:?}", i);
    }

    // iter borrows the values
    // example6 is equivalent to example 4 without the & and *
    println!("{:?}", a);
    println!();
}

fn example7() {
    println!("Example7:");
    let a = [Point(1,2), Point(3,4), Point(5,6)];

    println!("{:?}", a);
    for i in a.into_iter() {
        print_type_of(&i);
        println!("{:?}", i);
    }

    // into_iter consumes the values
    // example7 is equvialent to example 2.
    // println!("{:?}", a);
    println!();
}


fn main() {
    example1();
    example2();
    example3();
    example4();
    example5();
    example6();
    example7();
}
