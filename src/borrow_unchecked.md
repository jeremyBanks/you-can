Unbinds the lifetime in a [reference][REF] (`&T` or `&mut T`). This disconnects
the input lifetime from the output lifetime, so use of the output reference will
no longer cause input reference to be kept "live" (as defined in
[RFC 2094: non-lexical lifetimes][NLL]). The output reference is given an
[unbounded lifetime][UBL]. This causes the borrow checker to effectively ignore
it. The output lifetime may still be implicitly bound by any references within
the referenced type `T`. You may need to unbind these as well, or replace them
with `'static`.

# Safety

_The section intentionally left blank._

# Example

Here we create two (aliasing) unbounded mutable references at once, and then
continue to use them even after the original value is dropped (and its true
lifetime has ended).

```rust
let mut x = 0;

let mut_1 = unsafe { ::you_can::borrow_unchecked(&mut x) };
let mut_2 = unsafe { ::you_can::borrow_unchecked(&mut x) };

drop(x);

*mut_1 = 1;
*mut_2 = 2;

assert_eq!(*mut_1, *mut_2, "I hope LLVM is in a good mood!");
```

[REF]: https://doc.rust-lang.org/std/primitive.reference.html
[NLL]: https://rust-lang.github.io/rfcs/2094-nll.html
[UBL]: https://doc.rust-lang.org/nomicon/unbounded-lifetimes.html
