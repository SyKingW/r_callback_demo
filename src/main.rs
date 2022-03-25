use std::{
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    println!("Hello, world!");

    base_use();

    println!("\n\n");

    thread_use();
}

/// 基础用法
fn base_use() {
    // 闭包作为参数
    parameter_is_callback(Box::new(|| {
        println!("回调 parameter_is_callback");
    }));

    // 泛型写法
    parameter_is_callback_t(Box::new(|| {
        println!("回调 parameter_is_callback_t");
    }));

    // 闭包放在结构体属性
    let mut t = FnTest {
        data: 0,
        callback: Box::new(callback_function), // 可以直接传入一个 function
        callback_mut: Box::new(|v| {
            // 可以直接编写闭包
            println!("FnMut: {}", v);
        }),
        callback_once: Box::new(|v| {
            println!("FnOnce: {}", v);
        }),
    };

    (t.callback)(1);
    (t.callback)(2);

    (t.callback_mut)(1);
    (t.callback_mut)(2);

    (t.callback_once)(1);

    // 一般来说，使用闭包大部分都是异步操作
    // 但是目前写法，移动到其他线程使用会报错，因为没有遵守 Send trait
    // thread::spawn(move || {
    //     (t.callback)(1);
    //     (t.callback_mut)(1);
    //     (t.callback_once)(1);
    // });
    /*
    想移动到其他线程使用，会报如下错误

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
     */
}

/// 异步线程用法
fn thread_use() {
    // 闭包作为参数，异步调用
    /*
    输出log顺序如下
    parameter_is_callback_thread start
    parameter_is_callback_thread end
    parameter_is_callback_thread thread start
    回调 parameter_is_callback_thread
    parameter_is_callback_thread thread end
     */
    parameter_is_callback_thread(Box::new(|| {
        println!("回调 parameter_is_callback_thread");
    }));
    // 等一下，只是为了输出log和下面异步错开区分而已
    sleep(Duration::from_millis(100));

    // 所以这里增加了 Send 这个 trait
    let mut thread_t = FnThreadTest {
        callback: Box::new(callback_function),
        callback_mut: Box::new(|v| {
            println!("Thread FnMut: {}", v);
        }),
        callback_once: Box::new(|v| {
            println!("Thread FnOnce: {}", v);
        }),
    };

    /*
    以下执行顺序是
    main start thread
    main end(先执行完当前线程)
    thread start(开始执行线程里面的)
    Fn callback_function: 1
    Thread FnMut: 1
    Thread FnOnce: 1
    thread end
     */
    println!("main start thread");

    thread::spawn(move || {
        println!("thread start");
        (thread_t.callback)(1);
        (thread_t.callback_mut)(1);
        (thread_t.callback_once)(1);
        println!("thread end");
    });

    println!("main end");

    // 等待1秒，不然程序结束了，上面的 thread::spawn 还没运行完
    sleep(Duration::from_secs(1));
}

// 闭包作为参数
fn parameter_is_callback(callback: Box<dyn FnOnce() -> ()>) {
    println!("parameter_is_callback start");
    callback();
    println!("parameter_is_callback end");
}

// 当然挺多人用泛型写法...只能说，开心就好，理解之后，喜欢那种用那种
fn parameter_is_callback_t<T>(callback: Box<T>)
where
    T: FnOnce() -> (),
{
    println!("parameter_is_callback_t start");
    callback();
    println!("parameter_is_callback_t end");
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

fn callback_function(v: u8) {
    println!("Fn callback_function: {}", v);
}

struct FnTest {
    // 写结构体，除了闭包一般也会写一些需要传下去的数据，这里就写个 data 做一个示例而已
    data: u8,

    callback: Box<dyn Fn(u8) -> ()>,
    callback_mut: Box<dyn FnMut(u8) -> ()>,
    callback_once: Box<dyn FnOnce(u8) -> ()>,
}

struct FnThreadTest {
    callback: Box<dyn Fn(u8) -> () + Send>,
    callback_mut: Box<dyn FnMut(u8) -> () + Send>,
    callback_once: Box<dyn FnOnce(u8) -> () + Send>,
}
