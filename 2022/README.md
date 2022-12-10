# 2022
I am finally doing a Rust and here is what I learned.
I'd been planning to put these on Github eventually but finally felt encouraged to just share after seeing [@simonw's similar effort to use AoC to learn Rust on Github](https://github.com/simonw/advent-of-code-2022-in-rust/issues).

## Day 1: Calories
- Baby's first Rust program
- Having to import `std::io::BufRead` and discover `io::stdin().lock().lines()` to read input seemed overkill to me
- We can `parse()` strings to (an option that may have some) numbers
- We `unwrap()` an awful lot to implicitly explode on suboptimal `Option`
- Excited to already need `Vec::new()`
- Tailing the vector was infuriating until I discovered `split_off` which was very nice

## Day 2: Rock Paper Scissors
- I misread Rock Scissors Paper and miscoded the enum
- I actually played with this one quite a bit after submitting:
    - Wrote my first `HashMap`, then converted everything to `enum` types
    - Needed to use `#[derive(Copy, Clone, PartialEq)]` to do nice things to the enum
    - Implemented the `std::str::FromStr` trait on my enums
- Wrote my first `match`, which seems to be a big deal in Rust
- Everything is `unwrap()`, presumably I should be doing this more Rusty

## Day 3: Backpacks
- Wrote my first proper function and it doesn't even have a `return` statement
- `retain()` is a nice function that filters a `HashSet` in place on a predicate

## Day 4: Overlaps
- Imported the `regex` crate
- Learned that `min` and `max` are not in the prelude!
- `get(i).unwrap().as_str().parse().unwrap()` feels not good

## Day 5: Crates and cranes
- Surprised how difficult just getting the n'th character of a String was (but understand why)
- Learned a bit about lifetimes here as this was the first time I needed to keep hold of a string for longer than a loop iteration
    - Not sure I've even solved it correctly but I made a `Vec<String>` and inserted `line.clone()`
- Parsing the stupid stack spec took longer than solving the problem itself as I was trying to be clever
- Made a vector of vectors: `Vec<Vec<char>>`: woah!
- Prefixing tuple elements with `&` when iterating will deref automatically
- `extend_from_slice` was a nice way to extend one stack with the `split_off` of another

## Day 6: Comms
- Solved this the way I would have in Python but it feels a bit less elegant that the equivalent
- Easily my fastest and most succinct solve so far
- Used `std::env::args` to read my first argument from the CLI!

## Day 7: File system
- Immediately decided I wasn't doing a tree
- Made a right mess of `String` and `&str` in this one
    - I'm calling `to_string()` a lot just to make it work, not because I necessarily want a String
- Getting comfortable with the basic collections and I'm even starting to remember to `mut`
- Big surprise with collections having multiple types of iterator
    - Kept getting ownership moved errors
    - `iter_mut()` is a very nice way to iterate over a `HashMap` and modify values directly

## Day 8: Trees
- Discovered ranges where start > end silently do nothing, need to call `rev()`
- Why are negative numbers so painful? They seem to be a poison that causes `isize` to spread everywhere?
    - Suspect I am missing something here...
- My initial solution was less clean but I enjoyed refactoring the four searches to one afterwards
- Enjoyed [this blogpost](https://fasterthanli.me/series/advent-of-code-2022/part-1) describing Rust as "working out how to get from one type to another" which makes me feel better about spending a lot of time working out how to do exactly that

## Day 9: Knots
- Aptly named, I freaked out about this one but it was actually quite straightforward once you understood the rules
- My solution for part one unnecessarily considered the direction of travel which invalidated the method on part two, which gave me a chance to clean up
- Diagnosing this required writing a bunch of iterations over arrays and vectors which was useful learning
- Remembered tuple destructing to get a mutable ref destructured `let (x, y) = &mut vec[0];`
- Used a `HashSet` of tuples!
- Negative numbers strike again
- Was getting worried that my Rust was not very idiomatic
    - Discovered `cargo clippy` which offers wonderful suggestions and encouragement for writing `match` statements

## Day 10: Pixels
- My biggest trouble today was not reading the question carefully enough, so it's almost like regular programming
- Tried out `rustfmt` which seems to be Rust's equivalent of Python's Black except presumably nobody argues about it
