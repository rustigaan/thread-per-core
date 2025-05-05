# Thread per core

I read [Notes on structured concurrency, or: Go statement considered harmful](https://vorpus.org/blog/notes-on-structured-concurrency-or-go-statement-considered-harmful/) and [Async Rust can be a pleasure to work with (without \`Send + Sync + 'static\`)](https://emschwartz.me/async-rust-can-be-a-pleasure-to-work-with-without-send-sync-static/) and I'd like to try this for real.

I'm not running Linux, however, so I can't use `glommio`. The Tokio-based examples that Evan shows are single-threaded, and I'd like to be able to use all the cores in my CPU.

Then I stumbled upon a crate called `core_affinity` and the idea occurred to me to use it to spin up a thread per core, tie the thread to the core and create a single threaded Tokio-runtime for each thread. These threads can consume tasks from a channel and use properly scoped concurrency primitives and non-`Send` Futures to carry them out.
