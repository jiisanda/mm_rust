# RefCounter (RC)

```rust
use std::{cell::RefCell, rc::Rc};

pub fn share_the_ring() {
    let saurons_ring = MyPreciousRing::forge();
    println!("Sauron's ring saya: {saurons_ring:?}");

    // somehow, it reaches frodo
    let fordos_ring = Rc::new(saurons_ring);
}
```

RC is a RefCounter, we all know there can only be one owner, however if we put your memory in RC, you have a RefCounter
and essentially a lot of people can have a RefCounter and this RefCounter can internally can reference a same object. So
somehow we end up with one memory location, all the RefCounters can point to this one location.

So what out ownership rule, In Rust there is a thing called Unsafe programming, where we can do anything like
create pointers, etc.

So we now have a RefCounter, we can have a clone of this RefCounter, and another clone, ... and all these RefCounters 
point to the same memory location. The ownership principle this applies to the RefCounter, if RefCounter goes out of scope
it will get destroyed, and it will decrement the internal reference count, so after all the RefCounters are destroyed Rust
will automatically deallocate the memory.

### What can we do with RefCounter?

Well we can, if we want to take a look at the RefCounter.

```rust
println!("The ring now has {} owner.", Rc::strong_count(&fordos_ring));         // o/p: 1
```

the method `strong_count` will give us the number of references to our ring.

```rust
let samwises_ring = Clone::clone(&fordos_ring);
println!("Keep it safe... ({samwises_ring:?})");
```

Here, we are not cloning the ring but cloning the RefCounter, what does that mean? we get a tiny object on our stack which
is reference to a memory location where the RefCounter lives, and this new variable `samwises_ring` on our stack, will 
increase the RefCounter by 1. So, now if we do `strong_count` we get output `2`. If this `Rc` object goes out of scope 
at the end of the method, it will decrement the reference count.

And if we try printing, the reference of both the rings `frodos_ring` and `samwise_ring`, we get they are both pointing
the same memory location.

```rust
println!(
        fordos_ring.as_ref(),           // 0x225a1bcfd70
        samwises_ring.as_ref()          // 0x225a1bcfd70
    );
```
