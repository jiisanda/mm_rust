#[derive(Debug)]
pub struct MyPreciousRing {
    pub engraving: String
}

impl MyPreciousRing {
    pub fn forge() -> Self {
        Self {
            engraving: "One Ring to rule them all.".to_string()
        }
    }
}

pub fn ownership_ring() {
    let saurons_ring = MyPreciousRing::forge();
    println!("Souron's ring says: {saurons_ring:?}");

    let gollums_ring = saurons_ring;
    println!("My precious... ({gollums_ring:?})");

    let bilbos_ring = gollums_ring;     // again we have the transfer of ownership

    let frodos_ring = bilbos_ring;
    println!("Have to destroy it... ({frodos_ring:?})");

    let samwises_ring = &frodos_ring;
    println!("keep it safe... ({samwises_ring:?})");
    // println!("Have to destroy it... ({frodos_ring:?})");         // works perfectly fine

    let mut frodos_ring = frodos_ring;      // shadowing
    heat(&mut frodos_ring);
    // println!("Have to destroy it... ({frodos_ring:?}");             // frodo still has ownership of the ring

    // NOTE: there could be multiple borrows but only one "mutable borrow" at the same time
    // let mut frodos_ring_mut = &mut frodos_ring;         // error
    // let mut samwise_ring_mut = &mut frodos_ring;        // error
    // heat(&mut frodos_ring_mut);             // error
    // heat(&mut samwise_ring_mut);            // error

    destroy(frodos_ring);
    // println!("Have to destroy it... ({frodos_ring:?}");             // error as frodo has lost the ownership of the ring
}

// helper function doing mutable borrow
pub fn heat(ring: &mut MyPreciousRing) {
    ring.engraving = "Ash nazg durbatulûk, ash nazg gimbatul, ash nazg thrakatulûk agh burzum-ishi krimpatul.".to_string();
}

// this function is similar to drop() which does noting but transfer the ownership to some random variable which when
// goes out of scope the variable gets destroyed, i.e., it has no owner and memory is deallocated.
/*
Following is the drop fn.
pub fn drop<T>(_x: T) {}
 */
pub fn destroy(_ring: MyPreciousRing) {
    println!(r"
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣤⣶⣶⣶⣶⣄⠀⢠⣄⡀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣾⣿⣿⡿⠛⢻⣿⣿⣿⠀⢀⣿⣿⣦⡀⠀⠀
    ⠀⠀⠀⠀⠀⠀⣠⣴⣿⣿⣿⠋⠉⠁⠀⣸⣿⣿⡏⠀⢸⣿⣿⣿⣷⡄⠀
    ⠀⠀⠀⠀⢀⣾⣿⣿⠋⠁⠉⠀⣰⣶⣾⣿⡿⠟⠀⢠⣿⣿⣿⣿⣿⣿⡄
    ⠀⠀⠀⣴⣿⣿⠟⠛⠀⠀⣿⣿⣿⡿⠛⠉⠀⠀⢠⣾⣿⣿⣿⣿⣿⣿⡇
    ⠀⢀⣾⣿⣿⠿⠀⠀⣶⣾⣿⡿⠋⠀⠀⠀⠀⣰⣿⣿⡟⠉⢻⣿⣿⣿⠇
    ⠀⣾⣿⡏⠀⢀⣀⣴⣿⡿⠋⠀⠀⠀⠀⣠⣾⣿⣿⠋⠁⠀⢀⣿⣿⡟⠀
    ⢸⣿⣿⣧⣀⣼⣿⣿⡟⠁⠀⠀⠀⣠⣾⣿⣿⠛⠛⠀⠀⣾⣿⣿⡟⠀⠀
    ⠸⣿⣿⣿⣿⣿⡿⠏⠀⠀⢀⣠⣾⣿⡿⠿⠿⠀⢠⣤⣾⣿⣿⠟⠀⠀⠀
    ⠀⠈⠉⠉⠁⠀⢀⣀⣤⣾⣿⣿⠿⠿⠃⠀⣀⣠⣾⣿⣿⡿⠃⠀⠀⠀⠀
    ⠀⠳⣶⣶⣶⣿⣿⣿⣿⣿⣿⣏⠀⢀⣀⣠⣿⣿⣿⡿⠋⠀⠀⠀⠀⠀⠀
    ⠀⠀⠙⢿⣿⣿⣿⣿⣿⣿⣿⣿⣶⣾⣿⣿⣿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠙⠻⢿⣿⣿⣿⣿⣿⣿⠿⠛⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⠈⠉⠉⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀")
}