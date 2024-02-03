#[allow(unused_imports)]
use std::{cell::RefCell, rc::Rc};

#[allow(unused_imports)]
use crate::lor::{heat, MyPreciousRing};

pub fn share_the_ring() {
    let saurons_ring = MyPreciousRing::forge();
    println!("Sauron's ring saya: {saurons_ring:?}");

    // somehow, it reaches frodo
    let fordos_ring = Rc::new(saurons_ring);        // see docs
    println!("The ring now has {} owner.", Rc::strong_count(&fordos_ring));
    println!("Have to destroy... {fordos_ring:?}");

    let samwises_ring = Clone::clone(&fordos_ring);
    println!("Keep it safe... ({samwises_ring:?})");
    println!("The ring now has {} owner.", Rc::strong_count(&fordos_ring));

    println!(
        "\t{:p}\n\t{:p}",
        fordos_ring.as_ref(),
        samwises_ring.as_ref()
    );

    // let mut fordos_ring = fordos_ring;
    // heat(&mut fordos_ring);                      // this will give an error as cannot have a mutable reference
}

pub fn share_and_alter() {
    let saurons_ring = MyPreciousRing::forge();

    // somehow gets to frodo
    let frodos_ring = Rc::new(RefCell::new(saurons_ring));
    println!("Have to destroy... ({:?})", frodos_ring.borrow());
    // if we want to have read-only reference, we simply say borrow, and we can have as many read-only borrows as we want
    // and the borrow() will check that at runtime

    // samwise comes and borrows and clones the ring here we are cloning the Rc and referencing to the same RefCell
    let samwises_ring = Clone::clone(&frodos_ring);
    println!("The ring now has {} owners", Rc::strong_count(&frodos_ring));
    println!("Have to destroy... ({:?})", frodos_ring.borrow());             // frodo can still access the ring.

    heat(&mut frodos_ring.borrow_mut());            // now we can have mutable borrow, and RefCell will check at runtime
    // that there is only one mutable borrow
    println!("The ring says: {:?}", samwises_ring.borrow());

    // Once this mutable borrow goes out of scope, then we can have another mutable borrow or say next read-only borrow
}
