# 16. Fearless Concurrency

## 16.1. Using Threads to Run Code Simultaneously

- In most current OS, an executed program's code is run in a `process`, and the operating system will manage multiple processes at once.
- Within a program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads.
- Because threads can run simultaneously, there's no inherent guarantee about the order in which parts of your code on different threads will run. This can lead to problems, such as:
  - `Race conditions`, where threads are accessing data in an inconsistent order
  - `Deadlocks`, where two threads are waiting for each other, preventing both threads from continuing
  - Bugs that happen only in certain situations and are hard to reproduce and fix reliably
- Rust standard library uses a 1:1 model of thread implementation, whereby a program uses one operating system thread per one language thread.

### Creating a New Thread with spawn

- To create a new thread, we call `thread::spawn` function and pass it to a closure containing the code we want to run in the new thread.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

- The call to `thread::sleep` force a thread to stop its execution for a short duration, allowing a different thread to run.
- The threads take turns, but isn't guaranteed: it depends on how OS schedules threads.

### Waiting for All Threads to Finish Using join Handles

- Above example stops spawned thread prematurely when main thread ends and there is no guarantee on which order threads run, so spawned thread might not run at all.
- We can fix by saving the return value, which is `JoinHandle` type into a variable.
- `JoinHandle` is an owned value that, when we call `join` method on it, will wait for its threads to finish.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

- Calling `join` on the handle blocks the thread currently running until the thread represented by the handle terminates.

### Using move Closures with Threads

- Often use the `move` keyword with closures passed to `thread::spawn` because the closure will then take ownership of the values it uses from the environment, thus transferring ownership of those values from one thread to another.
- By adding `move` keyword before the closure, we force the closure to take ownership of the values it's using rather than allowing Rust to infer that it should borrow the values.

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```
