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
println!("{:p}/t{:p}"
        fordos_ring.as_ref(),           // 0x225a1bcfd70
        samwises_ring.as_ref()          // 0x225a1bcfd70
    );
```

This concept in other languages may be referred to as smart pointers, but in rust it is RefCounters.

### Borrowing an Rc mutable

```rust
let mut frodos_ring = frodos_ring;
head(&mut frodos_ting);             // will give an error: cannot borrow data in an `Rc` as mutable
```

We know what is mutable borrow... that we are still the owner but we give the mutable borrow to someone else, that someone
else can write into the data structure, and this is what the `heat()` method does, it changes the engraving, but it is 
not allowed, why? The reason is simple, if we have a single object, and we have to borrow RefCounters, that reference all 
these object, if lot of people takes the mutable borrow on this memory location, we can easily run into a risk condition.
As all these owners of these RefCounters will write to this memory location, that would be end up into lot of problems.
So what is the solution for that...

#### RefCell

```rust
pub fn share_and_alter() {
    let saurons_ring = MyPreciousRing::forge();

    // somehow gets to frodo
    let frodos_ring = Rc::new(RefCell::new(saurons_ring));
}
```

`RefCell` is a reference to a memory, `RefCell` has a unique capability, it can check the ownership and borrowing rules
at runtime, not at compile time, so its essentially an ownership and borrowing checker, build into the compiler that does 
not get executed at compile time but at running time. If more than one location of program asks for the mutable borrow at
the same time it will say Nope!, but it will not check at compile time, because compiler cannot know, when somebody in our
program is asking for mutable borrow. And this is what RefCell is, and combining it with `ref_couter` we get something more
powerful a pattern which is called `internal mutability pattern`. We are having something that have multiple owners, with
RefCounting and still the capability to be borrowed in a mutable way.