# Ownership and Borrowing in Rust

The following code explains the ownership...

```rust
pub fn ownership_ring() {
    let saurons_ring = MyPreciousRing::forge();
    println!("Souron's ring says: {saurons_ring:?}");

    let gollums_ring = saurons_ring; 
    println!("My precious... ({gollums_ring:?})");
}
```

`let gollums_ring = saurons_ring` this is where ownership of saurons_ring is transferred to gollums_ring

and after this if Sauron again tries to access the ring rust will give out error

```rust
 println!("Souron's ring says: {saurons_ring:?}");
                                ^^^^^^^^^^^^^^^^ value borrowed here after move
```

So this was because the ownership was transferred, now ownership does not mean gollum has a certain permission on the ring, 
but it just means that now gollum is now responsible for __freeing the memory__. i.e, when owner goes out of scope the 
memory is freed. And this was the reason in or previous example the `Box` object was deallocated automatically, because the 
owner `point` got out of scope. 

```rust
pub fn ownership_ring() {
    
    // rest code 
    
    let frodos_ring = bilbos_ring;
    println!("Have to destroy it... ({frodos_ring:?})");

    let samwises_ring = &frodos_ring;
    println!("keep it safe... ({samwises_ring})");
}
```

Here `let samwises_ring = &frodos_ring;`, we do not have transfer of ownership, but Samwise is doing the __read-only borrow__
of the ring. i.e, Samwise does not have ownership, but can have read-only access to the ring, cannot change the ring. note: 
Frodo haven't lost the ownership of the ring. 

So the `&` before `&frodos_ring` shows the borrowing.
