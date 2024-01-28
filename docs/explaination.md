### Memory management with primitive datatype (say integer)

```rust
#[inline(never)]
pub fn playground() {
    let mut x = 42;
    x += 1;

    print_bytes(&x);
}
```

Here, lets see whats happening behind the scenes... what is happening to `x` is it getting stored in heap or in stack.

if we run the code as assembly language using `just asm playground`

```assembly
cargo asm mm_rust::playground
mm_rust::playground:
 sub     rsp, 40
 mov     dword, ptr, [rsp, +, 36], 43
 lea     rcx, [rsp, +, 36]
 call    mm_rust::print::print_bytes
 nop
 add     rsp, 40
 ret
```

Here,
Let's see...
In the line ` mov     dword, ptr, [rsp, +, 36], 43
` we have our answer `43` which is stored by the pointer `rsp`, which indicates stack pointer. So, our variable x lives on the stack. And when we call `print_bytes`, it uses `lea` operator, which loads the address of the variable `x` to register `rcx`, so we are passing variable by reference through this register `rcx`, which is pointer to our variable on the stack.
