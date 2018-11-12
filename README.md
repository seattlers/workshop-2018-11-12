# Concurrent Rust

## 1. Warm-up

In the workshop repo, we have a `sleeper` crate. This provides a program which does the following:

1. Randomly choose a number of milliseconds (between 10 and 1,000).
2. Sleep (do nothing) for the chosen amount of time.
3. Print the sleep time.

Modify this program so that it uses `std::thread` to spawn some number of
threads (say, 4), which each do this, with their own sleep time.

## 2. Shared-nothing Digestion

The `sha` example crate implements a binary that accepts a list of file paths as
arguments, computes the SHA-256 digest of each file (one at a time), and prints
out a hex representation of the file hash.

Using what you learned in #1, modify `sha` to spawn a thread for each file which
performs the above work.

## 3. Counting by the Book

Write a program that uses `std::thread` to spawn N threads which share a
counter. Have each thread increment the counter, then exit.

**Hint:** This is exactly the exercise from [TRPL2e
  16.3](https://doc.rust-lang.org/book/second-edition/ch16-03-shared-state.html).

## 4. Counteeeeng

We provide a program `countee` that reads a file and counts the occurrences of
the ASCII byte `e`/`0x65`.

Rewrite this to divide up the counting work into N pieces and share it among N
threads. Share a single counter between threads, as in Exercise 2. Have each
thread bump the counter when it sees an `e`.

Hints:
- A good way to read file data is via a `BufReader`.
- Each thread can have its own `BufReader`.
- To start reading a file at an offest, you can call `seek()` on a `BufReader`.
- The size of the file usually won't divide evenly into the number of threads.
  It will probably be easiest to have the thread that reads the last chunk
  (until EOF) be the thread that reads a little extra or a little less.


## 5. Channeleeeeing

Modify the program from #3 to use _channels_ from `std::mpsc`. Have each thread
count its section of the input file, send its result across the channel, and
have the receiver/consumer sum the sub-results.

**Hint:** Check out [TRPL2e 16.2](https://doc.rust-lang.org/book/second-edition/ch16-02-message-passing.html).

## 6. Counting up to 26

Take one of the programs in 3 or 4, and modify it to count all occurrences of
ASCII letters.

Feel free to interpret this however you like (e.g. count only lower or uppercase
letters, or lump them together).

The interesting thing about this exercise is choosing a data structure to store
the counts, and if needed, combine them.

## 7. Futuristic concurrency

This is a whole big can of worms, but we can at least get a taste of it,
building on top of what you've already done.

Rewrite the channel version of one of the above programs using Tokio and
futures, via `tokio_threadpool`.

Hints:
- Read up on [futures]( https://tokio.rs/docs/getting-started/futures/). From
  the Rust perspective, what is a `Future`?
- Read up on the Tokio [runtime](https://tokio.rs/docs/getting-started/runtime/) concept.
  What is the job of the the runtime, with respect to a `Future`?
- Think about what you are doing when "spawning a `Future` onto a runtime"
  vs. "spawning a thread". What's different?
- Take a look at this minimal example from the tokio-threadpool [README](https://github.com/tokio-rs/tokio/tree/master/tokio-threadpool#examples).
