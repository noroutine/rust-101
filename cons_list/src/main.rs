use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    // Cons(i32, Box<List>),
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
    let a = Rc::new(Cons(
        Rc::new(RefCell::new(5)),
        Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
    drop(c);
    println!("count after dropping c = {}", Rc::strong_count(&a));

    let value = Rc::new(RefCell::new(5));

    let aa = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let bb = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let cc = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    // usage 
    if let Cons(ref aa_val, ref _aa_next_list) = *aa {
        println!("{}", aa_val.borrow());
    }
    // Using val and next_list means reading the fields
    

    println!("aa after = {aa:?}");
    println!("bb after = {bb:?}");
    println!("cc after = {cc:?}");

    
}
