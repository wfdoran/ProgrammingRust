fn ref_it() {
    let v : Vec<_> = (101..106).map(|i| i.to_string()).collect();
    let x = &v[2];
    println!("{} {:?}", x, v);
}

fn clone_it() {
    let v : Vec<_> = (101..106).map(|i| i.to_string()).collect();
    let x = v[2].clone();
    println!("{} {:?}", x, v);
}

fn pop_it() {
    let mut v : Vec<_> = (101..106).map(|i| i.to_string()).collect();
    let x = v.pop().expect("vector empty");
    println!("{} {:?}", x, v);
}

fn replace_it() {
    let mut v : Vec<_> = (101..106).map(|i| i.to_string()).collect();
    let x = std::mem::replace(&mut v[2], 107.to_string());
    println!("{} {:?}", x, v);
}

fn swap_it() {
    let mut v : Vec<_> = (101..106).map(|i| i.to_string()).collect();
    let x = v.swap_remove(2);
    println!("{} {:?}", x, v);
}

fn consume_it_all() {
    let v : Vec<_> = (101..106).map(|i| i.to_string()).collect();
    for x in v {
        println!("{}", x);
    }
}

fn main() {
    ref_it();
    clone_it();
    pop_it();
    replace_it();
    swap_it();
    consume_it_all();
}
