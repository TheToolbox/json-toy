# Toolbox's JSON Parser

Just a fun little attempt to learn Rust while doing something (relatively) non-trivial. Not always the best code, but it was a good exercise. Feel free to add to it (although I'm not sure why you'd want to).

# Architecture

The parser is a state machine, which Rust's enum facilities lent themselves very keenly to. That being said, a way to specify only a subset of an enum as a type hint would have helped quite a bit, as there are several places where I would have liked to be able to say "Do this for JSON objects and arrays, but not primitives". I'm sure there's some cleaning up that could be done, however, to improve readability, performance, and correctness.

The parser assumes one of several states, spending most of its time in the `ExpectingItem` and `ExpectingComma` states. During `ExpectingItem`, the parser is waiting for a new data item (object/array/primitive), and thus consumes whitespace until a different character is detected, changing state to then parse the encountered item. `ExpectingComma` is the common state when inside objects and arrays, and thus consumes whitespace until either a comma or a `}` or `]` is encountered, indicating the closing of the object/array. After encountering a comma, `ExpectingComma` moves to `ExpectingItem`. 

The parser maintains an object stack to track which object it is currently operating inside of. Similarly, there is a stack of keys, as the key for a nested object need to be remembered until the nested object has been parsed fully and can be added with the key to its parent.

## Architectural improvement

It may be more appropriate and readable to pass an iterator around rather than use a for loop, and have separate functions or internal loops to deal with certain cases (ie string parsing) to skip the possibly expensive jump tables for the state type and character type when state is known not to change. CoRoutines would clearly be helpful here. Perhaps there is a pattern for this. I'll investigate at some point.

# Known flaws

 - Invalid Numbers can cause panics
 - Valid JSON numbers that use the '+' in the exponent are incorrectly rejected

# Benchmark
 
It runs acceptably. I can get ~90 MB/s on a surface pro 3, but I was only testing the same 8kB file over and over. JSON parsing speed is fairly difficult to standardize, as performance is unsurprisingly very tied to the context that the parser is used in. Regardless, it's fast enough that I wouldn't ignore it for performance reasons. There are plenty of other good reasons to not use it.

For comparison, the nodejs parser ran at ~160 MB/s ish.