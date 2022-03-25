# r_callback_demo

rust 闭包 demo(Fn/FnMut/FnOnce)


## 闭包作为参数


```rust
fn main() {
    parameter_is_callback_t(Box::new(|| {
        println!("回调 parameter_is_callback_t");
    }));
}

// 闭包作为参数
fn parameter_is_callback_t<T>(callback: Box<T>)
where
    T: FnOnce() -> (),
{
    println!("parameter_is_callback_t start");
    callback();
    println!("parameter_is_callback_t end");
}
```

## 闭包作为结构体属性

```rust
fn main() {
    let mut t = FnTest {
        callback: Box::new(callback_function), // 可以直接传入一个 function
        callback_once: Box::new(|v| {
            // 可以直接编写闭包
            println!("FnOnce: {}", v);
        }),
    };

    (t.callback)(1);
}

struct FnTest {
    callback: Box<dyn FnOnce(u8) -> ()>,
    callback_once: Box<dyn FnOnce(u8) -> ()>,
}

fn callback_function(v: u8) {
    println!("Fn callback_function: {}", v);
}
```

## 异步使用闭包

主要就是加 Send trait，没加 Send 会报如下错误

```
    error[E0277]: `dyn FnOnce(u8)` cannot be sent between threads safely
--> src\main.rs:29:5
    |
29  |     thread::spawn(move || {
    |     ^^^^^^^^^^^^^ `dyn FnOnce(u8)` cannot be sent between threads safely
    |
    = help: the trait `Send` is not implemented for `dyn FnOnce(u8)`
    = note: required because of the requirements on the impl of `Send` for `Unique<dyn FnOnce(u8)>`
    = note: required because it appears within the type `Box<dyn FnOnce(u8)>`
    = note: required because it appears within the type `[closure@src\main.rs:29:19: 33:6]`
note: required by a bound in `spawn`
xxx
    |
621 |     F: Send + 'static,
    |        ^^^^ required by this bound in `spawn`
```

```rust
fn main() {
    parameter_is_callback_thread(Box::new(|| {
        println!("回调 parameter_is_callback_thread");
    }));

    let mut thread_t = FnThreadTest {
        callback_once: Box::new(|v| {
            println!("Thread FnOnce: {}", v);
        }),
    };

    thread::spawn(move || {
        println!("thread start");
        (thread_t.callback_once)(1);
        println!("thread end");
    });

    // 等待1秒，不然程序结束了，上面的 thread::spawn 还没运行完
    sleep(Duration::from_secs(1));
}

// 闭包作为参数，并且异步调用
fn parameter_is_callback_thread<T>(callback: Box<T>)
where
    T: FnOnce() -> () + Send + 'static,
{
    println!("parameter_is_callback_thread start");
    thread::spawn(move || {
        println!("parameter_is_callback_thread thread start");
        callback();
        println!("parameter_is_callback_thread thread end");
    });
    println!("parameter_is_callback_thread end");
}

struct FnThreadTest {
    callback_once: Box<dyn FnOnce(u8) -> () + Send>,
}

```



