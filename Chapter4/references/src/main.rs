use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn show1(table: Table) {
    print_type_of(&table);
    for (key, value) in table {
        print_type_of(&key);
        print_type_of(&value);
        println!("{}", key);
        for x in value {
            println!("  {}", x);
        }
    }
}

fn show2(table: &Table) {
    print_type_of(&table);
    for (key, value) in table {
        print_type_of(&key);
        print_type_of(&key);
        println!("{}", key);
        for x in value {
            println!("  {}", x);
        }
    }
}


fn move_table() {
    let mut t = Table::new();
    t.insert("aaa".to_string(), vec!["a1".to_string(), "a2".to_string()]);
    t.insert("bbb".to_string(), vec![]);
    t.insert("ccc".to_string(), vec!["c1".to_string(), "c2".to_string(), "c3".to_string()]);

    show1(t);
    println!();
}

fn borrow_table() {
    let mut t = Table::new();
    t.insert("aaa".to_string(), vec!["a1".to_string(), "a2".to_string()]);
    t.insert("bbb".to_string(), vec![]);
    t.insert("ccc".to_string(), vec!["c1".to_string(), "c2".to_string(), "c3".to_string()]);

    show2(&t);
    println!();

    t.insert("ddd".to_string(), vec!["d1".to_string()]);

    show2(&t);
    println!();
}

fn see_thru() {
    let x = 10;
    let y = 20;

    let rx = &x;
    let rrx = &rx;

    let ry = &y;
    let rry = &ry;

    
    println!("{}", rry > rrx);
    println!("{}", ry > rx);
    println!("{}", *rry > rx);
    println!("{}", rry > &rx);
    println!("{}", rry > &&x);

    println!("{}", rx + &10 == y);
    println!("{}", rrx + &&10 == y);
}



fn main() {
    move_table();
    borrow_table();
    see_thru();
}