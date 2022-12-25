# 2022
I am finally doing a Rust and here is what I learned.
I'd been planning to put these on Github eventually but finally felt encouraged to just share my efforts after seeing [@simonw's similar effort to use AoC to learn Rust on Github](https://github.com/simonw/advent-of-code-2022-in-rust/issues). To see a true crab solve things in idiomatic Rust, check out [@fasterthanlime's excellent AoC 2022 series](https://fasterthanli.me/series/advent-of-code-2022).

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

## Day 11: Monkey business
- Is this advent of text parsing 2022?
    - Can't quite believe I got the annoying line parsing working...
    - Replaced it with Regex named capture groups so I could stomach to share it here
- Wrote my first `Struct`!

## Day 12: Hill climb
- Wrote my second `Struct` but this time with its own `impl` block
- Tried out the `priority-queue` crate
- Spent ages debugging the clever Dijkstra bit only to realise I'd messed up the height check with a classic off-by-one -- drawing results out to the screen is definitely helpful

## Day 13: List distress
- Advent of Text Parsing continues. I got stuck for a little while so wrote a borderline trivial solution in Python (using `ast.literal_eval` not `eval` ) to wrap my head around the rules and compare my results.
- Immediately had the right idea to use an `enum` that could contain a value or a `Vec` of itself which made me feel better about having to look at some solutions in the megathread for inspiration on actually implementing it
- Some String things:
    - Can use `String::new()` instead of `"".to_string()`
    - `my_str.push(my_char)` instead of `my_str += &my_char.to_string()`
    - `my_str.clear()` instead of `my_str = ...`
- Ordering
    - Needed to implement `PartialOrd` and `Ord`, used the Python implementation as a template to write a pattern match
    - `PartialEq` and `Eq` needed for ordering but can just be derived
- Easily the hardest problem so far (in Rust anyway...)

## Day 14: Sand
- A turning point today, I really enjoyed writing this program and didn't need to look up anything to get it working!
- Ran `cargo build --release` just to see how hilariously fast it would run on Part 2: `0.01s`!

## Day 15: Beacons
- Extremely wound up from not reading the examples carefully enough. I'd drawn a nice map and thought it did not compare to the solution, only to realise after 30 minutes that the solution map was now wider to show the extra blocked tiles beyond the initial co-ordinates. Luckily this pointed at a solution for Part 1!
- Notably, the question wound me up, not my Rust abilities. I think I've reached script-kiddie level.
- Had some fun with the `bit-vec` crate
- Returned to Part 2 afterward to swap the candidate edge HashSet for a Vec and decreased run time from 10s to sub 1s -- seems it is just cheaper to just handle the duplicates than control duplicates via hash
- Update: Came back to this and simplified further by not collecting the cells into a structure at all!

## Day 16: Valves
- Hard work! I'd noticed immediately we could ignore rooms that had a zero rate valve which made things much easier.
- Felt very smart using BFS to work out the distance of all valve pairs for later, but then took the wrong approach in trying to write a heuristic that got very close to the answer.
- Eventually realised I should have been running DFS to find the highest flow rate recursively. Clunkily got it working for the solve.
- Discovered the magic `?` shortcut for `unwrap()`
- Update: Came back to clean this up as the code was the worst of my AoC journey so far! There's some `clone` going on with my vectors because I don't understand the complexities of borrowing yet. The best addition here was bringing running time to sub 0.1s after realising I could sort the Part 2 paths and stop early if the score sum is too low!

## Day 17: Tetris
- Got up early and ranked 2000 for Part 1! Hardest part was working out how to define the shapes I wanted to use in Rust.
- Spent about 15 minutes trying to work out where I'd introduced an off-by-one error, only to discover I had allowed "increasing" the cave top height to values lower than the current height!
- Misread the big boi number and switched my Vector based map to use a VecDeque, pushing new lines onto the top and popping old lines from the bottom which was of course woefully inadequate for the true number!
- Fell asleep trying to detect cycles for Part 2 but came back to it later, and wrote my own cycle finding search which was quite fun, also put it in a module and learned to stick `pub` everywhere

## Day 18: Cubes
- So straightforward I felt like I was missing something the entire time! Intended to overcomplicate Part 1 with a search which would have been perfect for a faster solve in Part 2.
- Learned a neat trick for testing an element is in a range (by just making a range and using `contains`)
- Collapsed my transform loop into `map` and `collect`, very crab!
- Still having trouble remembering to pass `&` to things like `my_map.contains` (as it's not needed for inserting)

## Day 19: Robots
- Immediately recognised the need for DFS but fumbled the initial implementation so went outside and left it alone for a bit. Cleaned it up and got it running easily later in the day with a fresh mind. The problem space necessitated some pruning so I decided to always build a geode robot if possible (a controversial choice with the problem purists), otherwise obisidian, elsewise one of the three remaining options.
- The DFS was a little slow for Part 2 so added an extra rule to not build ore or clay robots if we're holding twice their cost (as we're obviously producing surplus) and got down to sub 1s. Quite pleased with my 10 minute part delta!
- Update: Tidied this up to abstract my resource counts into a Struct and defined `std::ops::Add` and `std::ops::Sub` to be able to do simple math ops on them

## Day 20: Mixing
- Was excited to try `std::collections::LinkedList`, only to discover that it doesn't support inserting into the middle so seems a bit pointless
- Thought about implementing my own but that's a too advanced level of effort for today...
- Went well other than failing to realise a `new_index` of 0 is the back of the list!
- Learned about `rem_euclid` for getting non-negative remainders

## Day 21: Monkey math
- Hilariously simple Part 1. Less simple Part 2.
- I'm actually really proud of this one: a nice mix of `struct`, `enum`, some `match` and the usual data structures of `HashMap` and `VecDeque`!
- Interested to see so many people went with searches instead of outright solving the equations backwards; though, I wrote Part 2 in Python first to get my head around it.
- But it really must be Christmas with this much `unwrap`'ing I'm doing...

## Day 22: Cube
- I am tired of parsing ASCII maps to be honest.
- Like most others I just hard coded the grid because this is supposed to be fun.
- I found Part 2 of this puzzle was hilariously difficult (even though in retrospect it should not have been); the scope for off by one errors and flipped signs was huge.
- Like almost everyone else, I ended up making an unweighted [companion cube](https://genomic.social/@samstudio8/109560113421695445) and deriving all the edge crossing transformations by hand.
- Finally realised after removing all the rocks from the input and using some of my own test data that I was not changing the turn direction correctly and finally reached a solve just before midnight.
- A little disappointed not to have learned anything Rusty after putting so much effort in, today was mostly just about coding tedious rules, but looking back over my solution I'm happy with what I did, which is something!

## Day 23: Planting
- Pretty straightforward and a significant relief after yesterday's bloodbath.
- Took me longer than I care to admit I had not stopped elves from moving infinitely!
- Enjoyed using my previous `Map` struct for reading and drawing a grid-based map, though I should have done this with HashMap.
- I had a gut feeling the input data was small enough to just use a grid, so I just set the grid to be big enough to hold the result and got on with my Christmas.

## Day 24: Blizzard
- Not my cleanest solve initially, but the `Map` struct that I've been reaching for in many of the puzzles this week continues to prove useful. Although I wish I'd implemented the HashMap for yesterday like I said I should have!
- I wasted a bunch of time trying to do BFS on the map before realising I needed to handle each minute state in lockstep to prevent the map getting out of sync with the search!
- I wish I'd just ignored the walls or altered the coordinates to start the field interior at (0,0) to remove a bunch of off-by-1 errors.
- Reading the solutions afterwards it seems I need to get better at recognising when to use LCM but manually moving the snowflakes around seems to work perfectly fine -- part 2 runs in 0.4s.
- Looking forward to this being over tomorrow!

## Summary
- I'm confident to say I've learned the very basics of Rust. I need to go off and learn how to handle negative numbers properly without slapping `as <u|i>size` everywhere, and learn how to keep `&str` instead of cloning `String` all over the place.
- More importantly, I actually enjoyed writing Rust! I think I am starting to see what the fuss is about and I'm looking forward to taking my skills to the next level.
- I found it's much easier to write crappy Python than crappy Rust. Explicit types (that actually matter), structs and safety all require more overhead that forces you to think more carefully about what you're doing. This helped me very well during AoC as often my Part 1 could be updated quite easily to handle Part 2 as I had to do it properly rather than quickly.
- I'd definitely do AoC again but I think I could have done without the pressure of wanting to rank on the private leaderboard.
