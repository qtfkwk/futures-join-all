/*!

Define a job composed of a common *task* (async function) to be run on a series of input values,
run it concurrently via [join_all][ja] on an implicit [ThreadPool][tp] (1 thread per logical CPU
core), and implicitly "collect" the ordered results into a vector

[ja]: https://docs.rs/futures/latest/futures/future/fn.join_all.html
[tp]: https://docs.rs/futures/latest/futures/executor/struct.ThreadPool.html

# Example

Build via `cargo build --release`, then run:

```text
$ ./target/release/futures-join-all 10
task 1 sleeping 9
task 2 sleeping 1
task 3 sleeping 5
task 4 sleeping 3
task 5 sleeping 9
task 6 sleeping 2
task 7 sleeping 8
task 8 sleeping 6
task 9 sleeping 6
task 10 sleeping 1
task 2 slept 1
task 10 slept 1
task 6 slept 2
task 4 slept 3
task 3 slept 5
task 8 slept 6
task 9 slept 6
task 7 slept 8
task 1 slept 9
task 5 slept 9

results = [(1, 9), (2, 1), (3, 5), (4, 3), (5, 9), (6, 2), (7, 8), (8, 6), (9, 6), (10, 1)]
```

This command spawns 10 tasks that respond immediately with the `sleeping` message and then begin
sleeping concurrently.
Each task prints its `slept` message when it is done and returns.
The results are collected and printed.

*/

use futures::future::join_all;
use futures::executor::block_on;

// Only used for `sleep()`:
use rand::Rng;
use std::time::Duration;
use async_std::task::sleep;

// Only used for the CLI
use std::env::args;
use std::process::exit;

/**

Common task that does *something* with an input value and returns a result

For demonstration purposes we just sleep for a random number of seconds between 1 and 10 in
[non-blocking fashion][nbf] via `async_std::task::sleep` to *simulate* a more time-consuming
calculation and then return the input and seconds slept in a tuple, but *you could do almost
anything here.*

[nbf]: https://blog.hwc.io/posts/rust-futures-threadsleep-and-blocking-calls-inside-async-fn/

*/
async fn task(n: u64) -> (u64, u64) {

    // Sleep for a random number of seconds
    let mut rng = rand::thread_rng();
    let secs: u64 = rng.gen_range(1, 11); // 1-10
    println!("task {} sleeping {}", n, secs);
    sleep(Duration::from_secs(secs)).await;
    println!("task {} slept {}", n, secs);

    // Return result
    (n, secs)
}

/**

Command line interface

*/
fn main() {

    // Process arguments
    fn usage() {
        eprintln!("\
Usage: `futures-join-all [OPTIONS] N`

* `N`: Number of tasks
* `OPTIONS`
    * `-h`, `--help`: Print usage
");
        exit(0);
    }
    let mut a = vec![];
    for arg in args().skip(1) {
        if ["-h", "--help"].contains(&arg.as_str()) {
            usage();
        } else {
            a.push(arg);
        }
    }
    if a.len() < 1 {
        usage();
    }
    let nums: Vec<u64> = a.iter().map(|x| {
        match x.parse::<u64>() {
            Ok(u) => u,
            _ => {
                eprintln!("ERROR: Failed to parse integer number of tasks from `{}`!", x);
                exit(1);
            },
        }
    }).collect();

    for num in nums {

        /*
        // Option 1: Define the whole job as a self-contained unit
        let job = async { join_all((1..=num).map(|x| task(x))).await };
        let results = block_on(job);
        */

        /*
        // Option 2: Define inputs separately and use the boilerplate job
        let inputs = 1..=num;
        let results = block_on(async { join_all(inputs.map(|x| task(x))).await });
        */

        // Option 3: Basically options 1 and 2 combined as a "one-liner"
        let results = block_on(async { join_all((1..=num).map(|x| task(x))).await });

        /*
        // Option 4: Use an async closure (currently unstable)
        // (https://github.com/rust-lang/rust/issues/62290) instead of an async function
        let results = block_on(async { join_all((1..=num).map(|x| (async |n| {
            println!("sleeping {}", n);
            sleep(Duration::from_secs(n)).await;
            println!("slept {}", n);
            n
        }).())).await });
        */

        // Print the results
        println!("\nresults = {:?}\n", results);

    }
}
