# Goal

Define a job composed of a common *task* (async function) to be run on a series of input values,
run it concurrently via [join_all][ja] on an implicit [ThreadPool][tp] (1 thread per logical CPU
core), and implicitly "collect" the ordered results into a vector

[ja]: https://docs.rs/futures/latest/futures/future/fn.join_all.html
[tp]: https://docs.rs/futures/latest/futures/executor/struct.ThreadPool.html

# Example

Build via `cargo build --release`, then run:

<script id="asciicast-353428" src="https://asciinema.org/a/353428.js" data-autoplay="1" data-loop="1" async></script>

This command spawns 10 tasks that respond immediately with the `sleeping` message and then begin
sleeping concurrently in a [non-blocking fashion][nbf] via `async_std::task::sleep` to *simulate* a
more time-consuming calculation.
Each task prints a `slept` message when it is done and returns.
The results are collected and printed.

[nbf]: https://blog.hwc.io/posts/rust-futures-threadsleep-and-blocking-calls-inside-async-fn/
