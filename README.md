Enables threads to synchronize the beginning or end of some computation.

## Examples

```rust
use std::thread;

let wg = WaitGroup::new();
let (tx, rx) = mpsc::channel();

for _ in 0..THREADS {
    let wg = wg.clone();
    let tx = tx.clone();

    thread::spawn(move || {
        wg.wait();
        tx.send(()).unwrap();
    });
}

thread::sleep(Duration::from_millis(100));

// At this point, all spawned threads should be blocked, so we shouldn't get anything from the channel.
assert!(rx.try_recv().is_err());

wg.wait();

// Now, the wait group is cleared and we should receive messages.
for _ in 0..THREADS {
    rx.recv().unwrap();
}
```

## Implementations

WaitGroup 对象有一个 `count: Mutex<usize>` 的域用于记录有多少个 cloned 对象 (含 the original one). WaitGroup 实现了 Drop Trait，当对象 drop 时，`count -= 1`.

WaitGroup 提供了 `wait` 方法用于同步线程，调用 `wait` 时会 drop 当前对象，并当且仅当 count == 0 时返回。