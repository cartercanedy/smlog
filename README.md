# smlog: a lightweight and *very* basic stdout logging implementation

Useful for basic human-readable logging, and not much else

Rather than other logging implementations, this logger is extremely opinionated for the sake of
simplicity. The lack of choice means that you should be able to call `smlog::Log::init(...)`
once in main and immediately start logging using `log`'s logging macros, without adding a
significant amount of bloat to your project.

## Log formatting

Your log statements will look extremely basic:

* `info!(...)` prints `info: ...`
* `warn!(...)` prints `warning: ...`
* `error!(...)` prints `error: ...`
* `debug!(...)` prints `debug: ...`
* `trace!(...)` prints `trace: ...`

but sometimes, that's all you really need.

## Examples:

### Basic logging

```rs
use smlog::{log::{info, error}, Log, LevelFilter};

fn main() {
    Log::init(LevelFilter::Info);

    info!("Hello, world!");
    // Outputs `info: Hello, world!`

    error!("Goodbye, world!");
    // Outputs `error: Goodbye, world!`
}
```
