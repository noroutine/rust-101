use futures::executor::LocalPool;
use futures::task::LocalSpawnExt;
use trpl::Either;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, Instant};

// Let's create a custom future to see what's happening
struct MyFuture {
    completed: bool,
}

impl MyFuture {
    fn new() -> Self {
        println!("🔨 MyFuture::new() - Future created (but not started yet!)");
        Self { completed: false }
    }
}

impl Future for MyFuture {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("📊 MyFuture::poll() called");

        if !self.completed {
            println!("   ⏳ First poll - marking as completed");
            self.completed = true;
            println!("   🔄 Returning Poll::Ready");
            Poll::Ready("Hello from MyFuture!".to_string())
        } else {
            // This shouldn't happen for this simple future
            println!("   ⚠️  Already completed (shouldn't happen)");
            Poll::Ready("Already done".to_string())
        }
    }
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

fn main() {
    println!("🚀 Starting execution breakdown\n");

    // Step 1: Create the executor (just a task queue + polling mechanism)
    println!("1️⃣  Creating LocalPool (executor)");
    let mut pool = LocalPool::new();
    println!("   ✅ LocalPool created - it's empty right now\n");

    // Step 2: Get a spawner (handle to add tasks to the executor)
    println!("2️⃣  Getting spawner");
    let spawner = pool.spawner();
    println!("   ✅ Spawner obtained - can now add tasks\n");

    // Step 3: Create and spawn a future
    println!("3️⃣  Creating and spawning future");

    // Let's see both an async block and custom future
    spawner
        .spawn_local(async {
            println!("   📦 async block: Starting execution");
            println!("   📦 async block: About to await custom future");

            let result = MyFuture::new().await;
            println!("   📦 async block: Got result: {}", result);
            println!("   📦 async block: Finished");
        })
        .unwrap();

    println!("   ✅ Future spawned and added to executor's queue\n");

    // At this point, nothing has run yet! The future is just sitting in the queue.
    println!("🔍 Current state: Future is created and queued, but not executed\n");

    // Step 4: Run the executor
    println!("4️⃣  Calling pool.run() - this starts the execution loop");
    println!("   🔄 Executor will now poll all queued futures until completion\n");

    pool.run();

    println!("\n✨ All done! Executor finished when all futures completed");

    // book code
    trpl::run(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        println!("trying some channels");
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("tx1: hi"),
                String::from("tx1: from"),
                String::from("tx1: the"),
                String::from("tx1: future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("tx: more"),
                String::from("tx: messages"),
                String::from("tx: for"),
                String::from("tx: you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
            vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];

        trpl::join_all(futures).await;

        let slow_fut = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };

        let fast_fut = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };

        trpl::race(slow_fut, fast_fut).await;

        println!("race with sleep");
        let start = Instant::now();
        let one_s = Duration::from_secs(1);
        let a_fut = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::sleep(one_s).await;
            slow("a", 10);
            trpl::sleep(one_s).await;
            slow("a", 20);
            trpl::sleep(one_s).await;
            println!("'a' finished.");
        };
        let b_fut = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::sleep(one_s).await;
            slow("b", 10);
            trpl::sleep(one_s).await;
            slow("b", 15);
            trpl::sleep(one_s).await;
            slow("b", 350);
            trpl::sleep(one_s).await;
            println!("'b' finished.");
        };

        trpl::race(a_fut, b_fut).await;
        let time = Instant::now() - start;
        println!(
            "'sleep' version finished after {} seconds.",
            time.as_secs_f32()
        );


        println!("race with yield_now");
        let start = Instant::now();
        let ay_fut = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let by_fut = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };

        trpl::race(ay_fut, by_fut).await;
        let time = Instant::now() - start;
        println!(
            "'yield' version finished after {} seconds.",
            time.as_secs_f32()
        );

        println!("benchmarks");
        let one_ns = Duration::from_nanos(1);
        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::sleep(one_ns).await;
            }
        }
        .await;
        let time = Instant::now() - start;
        println!(
            "'sleep' version finished after {} seconds.",
            time.as_secs_f32()
        );

        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::yield_now().await;
            }
        }
        .await;
        let time = Instant::now() - start;
        println!(
            "'yield' version finished after {} seconds.",
            time.as_secs_f32()
        );

        async {
            println!("async with await");
        }.await;
    });

    println!("future with timeout");
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}
