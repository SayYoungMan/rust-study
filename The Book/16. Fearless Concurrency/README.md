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

## 16.2. Using Message Passing to Transfer Data Between Threads

- One approach to ensure safe concurrency is `message passing`, where threads or actors communicate by sending each other messages containing data.
- To accomplish this, Rust's standard library provides `channels`, which is a programming concept by which data is sent from one thread to another.
- A channel has two parts: `transmitter` and `receiver`. One part of your code calls methods on the transmitter with the data you want to send and another part checks the receiving end for arriving messages.

  - A channel is said to be `closed` if either transmitter or receiver is dropped.

- We create a new channel using `mpsc::channel` function; `mpsc` stands for `multiple producer, single consumer`.
  - Rust's standard library implements channels in a way that they can have multiple sending ends that produce values but only one receiving end that consumes the values.
- `mpsc::channel` function returns a tuple, the first element is the sending end (transmitter) and the second element is the receiving end (receiver).

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

- The transmitter has a `send` method that takes the value we want to send and return `Result` type, so if the receiver has already been dropped, it will return error.
- Receiver has `recv` method for blocking the main thread's execution and wait until a value is sent down the channel. It will then return `Result` as when the transmitter is closed, `recv` will return an error to signal that no more values will come.
- `try_recv` method doesn't block but will return `Result` immediately: `Ok` value holding a message if one is available and `Err` if no message. It's useful if this thread has other work to do while waiting for messages.

### Channels and Ownership Transference

- We can't use `val` anymore, in the previous example, after we've sent it down the channel via `tx.send`.
- Because once it's been sent, that thread could modify or drop it and it can cause unexpected results due to inconsistent or nonexistent data.

### Sending Multiple Values and Seeing the Receiver Waiting

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

- This time, the spawned thread has a vector of strings and we send them one by one.
- We are no longer calling `recv` function and we're treating `rx` as an iterator. When the channel is closed, iteration will end.

### Creating Multiple Producers by Cloning the Transmitter

```rust
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // --snip--
```

- This will give us two threads, each sending different messages to the one receiver.

## 16.3. Shared-State Concurrency

- Shared memory concurrency is bad because it's like multiple ownership: multiple threads can access the same memory location at the same time.

### Using Mutexes to Allow Access to Data from One Thread at a Time

- `Mutex` is an abbreviation for `mutual exclusion`, a mutex allows only one thread to access some data at any given time.
- To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex's `lock`.
- The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data.
- You must remember two rules with mutex:
  1. You must attempt to acquire the lock before using the data.
  2. When you're done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

### The API of `Mutex<T>`

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
```

- Create `Mutex<T>` using the associated function `new`.
- To access the data inside the mutex, we use `lock` method to acquire the lock. This call will block the current thread so it can't do any work until it's our turn to have the lock.
- The call to `lock` would fail if another thread holding the lock panicked.
- After we've acquired the lock, we can treat the return value, as a mutable reference to the data inside.
- The call to `lock` returns a smart pointer called `MutexGuard`, wrapped in a `LockResult`.
- The `MutexGuard` implements `Deref` to point at our inner data and has `Drop` implementation that releases the lock when it goes out of scope.

### Multiple Ownership with Multiple Threads

- `Rc<T>` is not safe to share across threads. When `Rc<T>` manages the reference count, it adds to the count for each call to `clone`and subtracts from the count when each clone is dropped.
- But it doesn't use any concurrency primitives to make sure that changes to the count can't be interrupted by another thread.

### Atomic Reference Counting with `Arc<T>`

- `Arc<T>` is a type like `Rc<T>` that is safe to use in concurrent situations.
- It stands for `atomically reference counted` type.
- Atomics work like primitive types but are safe to share across threads.
- The thread safety comes with a performance penalty so we don't need it on single thread.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

### Similarities Between `Refcell<T>`/`Rc<T>` and `Mutex<T>`/`Arc<T>`

- `Mutex<T>` provides interior mutability as the `Cell` family does.
- `Mutex<T>` comes with the risk of creating `deadlocks` like when `Rc<T>` creates reference cycles.
