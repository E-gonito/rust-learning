# Chapter 4: Ownership

Progress: 4.1 mostly done. Next is the end of 4.1 ("Return Values and Scope"), then 4.2 (References). 4.3 (Slices) is skipped for now.

## The three rules

1. Each value has one **owner**.
2. There is only one owner at a time.
3. When the owner goes out of scope, the value is **dropped** (its memory is freed).

## Stack and heap

- **Stack:** LIFO (last in, first out), like a stack of plates. It holds fixed-size values. A whole frame is freed at once when it is popped.
- **Heap:** for values whose size can change. The **allocator** finds a free block and returns a **pointer** to it.
- A `String` has two parts:

```
stack: s1 = { ptr, len: 2, cap: 2 }
heap:  [ 'h', 'i' ]
```

## What `=` does

| Code | What happens | Is the old variable still usable? |
|---|---|---|
| `let y = x;` (integer) | **Copy**: a new, separate value | Yes |
| `let s2 = s1;` (String) | **Move**: only ptr, len and cap are copied, and `s2` becomes the owner | No, compile error |
| `let s2 = s1.clone();` | **Clone**: the heap bytes are copied too | Yes |

- **Why moves exist:** if two owners pointed at the same heap bytes, both would free them. That is a **double free**, which is undefined behaviour.
- **Why `.clone()` must be written out:** copying heap bytes costs time and memory, so Rust makes the cost visible in your code.

## Functions move values too

```rust
fn takes(t: String) { println!("{t}"); }

let s = String::from("hi");
takes(s);          // works like: let t = s;
println!("{s}");   // error: s was moved
```

When `takes` ends, its frame is popped, `t` goes out of scope, and `drop` runs and frees the heap bytes.

> The stack does not free heap memory on its own. **`drop` does**, and it runs because the owner went out of scope. That is how Rust frees memory with no garbage collector and no manual `free`.

## Misconceptions I had

- I called the stack FIFO. It is **LIFO**.
- I thought `let y = x` with an integer would fail. Integers are **Copy**, so you get two separate values.
- I thought passing a `String` to a function kept it usable in `main`. It is a **move**.

## Review questions

Answer each one before you open it.

<details><summary>1. Does <code>let x = 1; let y = x; println!("{x}");</code> compile?</summary>

Yes. Integers are Copy, so `y` gets its own separate `1`.
</details>

<details><summary>2. Same thing with a <code>String</code>. Does it compile?</summary>

No. The String is moved, `s1` is invalid afterwards, and using it is a compile error.
</details>

<details><summary>3. Which parts of a <code>String</code> are on the stack, and which are on the heap?</summary>

On the stack: ptr, len and cap. On the heap: the bytes of the text.
</details>

<details><summary>4. A function's frame is popped. Who frees the heap bytes?</summary>

`drop`. It runs because the owner went out of scope.
</details>

<details><summary>5. Why does <code>.clone()</code> have to be written out?</summary>

Copying heap bytes is expensive, so Rust makes the cost visible in your code.
</details>

## Open question (answer it before reading 4.2)

`takes(s)` moves `s`. Find three ways to keep using `s` in `main`:

1. With a method you already know
2. By having the function give the value back
3. With a "window" onto the value

<details><summary>Answers (only after you've tried)</summary>

1. `takes(s.clone())`
2. `let s = takes(s);`, where `takes` returns the `String`
3. A reference: `fn takes(t: &String)` and `takes(&s)`. This is section 4.2.
</details>
