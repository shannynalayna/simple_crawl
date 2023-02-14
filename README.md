## Simple Rust-y link crawler  

From the top-level of this repository, run the following commands to interact with the crawler.

```
$ cargo build
```

Once we have all artifacts built, we can begin the crawler daemon!  

```
$ cargo run --bin crawler_daemon
```

The daemon will occasionally print out messages to this console. To stop the daemon, use `ctrl-c`
from the running terminal window.

The crawler CLI is now ready to use! 

In a different terminal window, at the top-level of this repository, you may run the following commands: 

```
$ cargo run --bin crawl start <URL_TO_CRAWL>
```
This subcommand will start the crawler at your chosen domain.

```
$ cargo run --bin crawl stop
```
This subcommand will stop the crawler, if it is running. 

```
$ cargo run --bin crawl list
```
This subcommand will return the results of the last crawl.

NOTE: Crawler commands are best suited in this particular order.