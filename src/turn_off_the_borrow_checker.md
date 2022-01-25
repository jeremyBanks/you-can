[You can't "turn off the borrow checker" in Rust][OFF], and you shouldn't want
to. [Rust's references][REF] aren't pointers, and the compiler is free to
decimate code that tries to use references as though they are. If you need raw
pointer behaviour in Rust, don't use this, use
[Rust's actual raw pointers][PTR], which don't make the same aliasing guarantees
to the compiler. However, if you would like to pretend the borrow checker
doesn't exist **for educational purposes and never in production code**, this
macro that will suppress many (though not all) borrow checker errors in the code
it's applied to.

This shouldn't break any otherwise-valid code; the borrow checker doesn't affect
compilation output, only verify input validity. However, it will allow unsound
and unsafe nonsense that will fail unpredictably and dangerously. This **is not
safe to use**.

### Example

#### Without Macro

```compile_fail
fn main() {
    let mut owned = vec![1, 32];

    let mut_1 = &mut owned[0];
    let mut_2 = &mut owned[1];
    //~^ ERROR cannot borrow `owned` as mutable more than once at a time

    drop(owned);
    //~^ ERROR cannot move out of `owned` because it is borrowed
    let undefined = *mut_1 + *mut_2;
    println!("{undefined}");
}
```

#### With Macro

```rust
#[you_can::turn_off_the_borrow_checker]
fn main() {
    let mut owned = vec![1, 32];

    let mut_1 = &mut owned[0];
    let mut_2 = &mut owned[1];
    //~^ WARNING the borrow checker is suppressed for these references.

    drop(owned);
    let undefined = *mut_1 + *mut_2;
    println!("{undefined}");
}
```

## Explanation

The macro looks for references created in the code by use of the `&` or `&mut`
operators or the `ref` and `ref mut` bindings, and wraps them with our
[`borrow_unchecked()`] function to [unbind their lifetimes][UBL], causing the
borrow checker to effectively ignore them. If running on nightly, it adds new
warning diagnostic messages for every reference it modifies.

#### Expanded

```rust
fn main() {
    let mut owned = vec![1, 32];

    let mut_1 = unsafe { ::you_can::borrow_unchecked(&mut owned[0]) };
    let mut_2 = unsafe { ::you_can::borrow_unchecked(&mut owned[1]) };

    drop(owned);
    let undefined = *mut_1 + *mut_2;
    println!("{undefined}");
}
```

This approached is limited. It can't suppress errors resulting from the code
illegally composing lifetimes created elsewhere, or references created
implicitly. As a workaround, prefixing `&*` can sometimes be used to force an
explicit reference where one is needed, such as as in the example below.

### Example

```rust
#[you_can::turn_off_the_borrow_checker]
fn main() {
    let mut source = Some(1);
    let inner_mut = &*source.as_ref().unwrap();
    let mutable_alias = &mut source;

    source = None;
    *mutable_alias = Some(2);

    if let Some(ref mut inner_a) = source {
        match source {
            Some(ref mut inner_b) => {
                *inner_b = inner_mut + 1;
                *inner_a = inner_mut + 2;
            },
            None => {
                println!("none");
            },
        }
    }

    println!("{source:?}");
}
```

#### Expanded

```rust
fn main() {
    let mut source = Some(1);
    let inner_mut = unsafe { ::you_can::borrow_unchecked(&*source.as_ref().unwrap()) };
    let mutable_alias = unsafe { ::you_can::borrow_unchecked(&mut source) };

    source = None;
    *mutable_alias = Some(2);

    if let Some(ref mut inner_a) = source {
        let inner_a = unsafe { ::you_can::borrow_unchecked(inner_a) };

        match source {
            Some(ref mut inner_b) => {
                let inner_b = unsafe { ::you_can::borrow_unchecked(inner_b) };

                *inner_b = inner_mut + 1;
                *inner_a = inner_mut + 2;
            },
            None => {
                println!("none");
            },
        }
    }

    println!("{source:?}");
}
```

## Discussions

Here are some related discussions, mostly about why you shouldn't do this:

- <https://reddit.com/s9az4y>
- <https://internals.rust-lang.org/t/16001>
- <https://news.ycombinator.com/item?id=30031323>
- <https://twitter.com/pcwalton/status/1485718602233704452>

[OFF]: https://steveklabnik.com/writing/you-can-t-turn-off-the-borrow-checker-in-rust
[PTR]: https://doc.rust-lang.org/std/primitive.pointer.html
[REF]: https://doc.rust-lang.org/std/primitive.reference.html
[UBL]: https://doc.rust-lang.org/nomicon/unbounded-lifetimes.html
