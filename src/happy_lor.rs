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

mod we_are_all_friends_here {
    #[derive(Debug, Copy, Clone)]
    pub struct MyPreciousRing {
        engravings: &'static str
    }

    impl MyPreciousRing {
        pub fn forge() -> Self {
            Self {
                engravings: "One Ring to rule them all"
            }
        }
    }
}

pub fn happy_lor() {
    let saurons_ring = we_are_all_friends_here::MyPreciousRing::forge();
    println!("Sauron's ring says: {saurons_ring:?}");

    let gollums_ring = saurons_ring;        // this is no longer a transfer of ownership, as our ring has a
                                                            // copy trait this line has new semantics, this means lets copy
                                                            // the data bit wise
    println!("My precious... ({gollums_ring:?})");
    println!("Sauron's ring says: {saurons_ring:?}");
}
