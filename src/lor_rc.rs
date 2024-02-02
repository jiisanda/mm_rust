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
}
