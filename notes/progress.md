# Progress (Rust)

My spaced-review log. Claude reads this at the start of every session and updates it at the end.

A question moves to **Mastered** only after I answer it correctly twice, in different sessions.

## Where I am
- Ch 4.1 is mostly done. Next: answer the open question, skim the end of 4.1 ("Return Values and Scope"), then read 4.2 (References). Skip 4.3 for now.
- After that: skim 5.1 (structs), then start `movement.rs` in my Bevy game (Leyline Front).

## Mastered
_(nothing yet: everything below has been answered correctly once at most)_

## Answered correctly once
- The three ownership rules
- Move: `let s2 = s1` makes `s1` invalid
- A double free is why moves exist
- `.clone()` copies the heap bytes, and has to be written out because it is expensive

## Mistakes to re-check
- Called the stack FIFO. It is **LIFO**.
- Thought `let y = x` with an integer fails because of "one owner". Integers are **Copy**, so a new value is made.
- Thought passing a `String` to a function keeps it usable in `main` and keeps the heap bytes. It is a **move**: the parameter owns it, and `drop` frees the heap bytes when the function ends.
- Was vague on how the heap allocates. The allocator hands out a block and returns a pointer. CS:APP 9.9 covers it fully.

## Open question
`takes(s)` moves `s`. Find three ways to keep `s` usable: see the bottom of [ch04-ownership.md](ch04-ownership.md).

## Review queue
1. Does `let x = 1; let y = x; println!("{x}");` compile? Why?
2. The same with a `String`. Why is it different?
3. Which parts of a `String` are on the stack, and which on the heap?
4. A function's frame is popped. Who frees the heap bytes, and when?
5. Why must `.clone()` be written out?

## Session log
- 2026-11 (several sessions): ch 4.1 ownership, stack and heap, move, copy and clone.
