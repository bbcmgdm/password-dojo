This is an attempt at the password cracking dojo using Rust.

It uses a word list loaded from a file, provided as a command line argument.

```shell
$ cargo run --release -- leaked_passwords_v1.txt wordlist.txt
# or perhaps, after a `cargo build --release`
$ ./target/release/password-dojo leaked_passwords_v1.txt wordlist.txt
```

## Single-threaded version

This is in the `original` branch in the repository.

This was the first version implemented to prove that it worked. It loads the list of username/password pairs into a `HashMap`. It then iterates over the list of words provided on the command line, and for each word will test the hash against the values in the map.

Because the hashing algorithm removes so much entropy from the password, it will often find several possible passwords for the same hash.

## Concurrent version

To make this concurrent, I used the [Rayon](https://github.com/rayon-rs/rayon) concurrency library, and replaced the `HashMap` with a [`DashMap`](https://docs.rs/dashmap/latest/dashmap/). This is a library that provides a data structure fairly similar to the `HashMap` from the standard library but with better support for concurrency. This version splits the incoming word list into several threads managed by Rayon, which defaults to one per CPU core. It can complete much quicker than the single-threaded one, and the output is similar although in a different order from the single-threaded version.

Because the concurrent version uses several threads, and Rust's ownership model doesn't allow several of them to modify the `progress` variable at once, I had to remove the progress notifications from this version to avoid having to add complex code involving mutexes or that kind of thing which would be slower and not really warranted.

## Speed

All of these tests use the [`rockyou` word list](https://github.com/brannondorsey/naive-hashcat/releases/download/data/rockyou.txt), which finds passwords for 60 of the leaked hashes.

### BBC MacBook Pro 2015, i7-4870HQ CPU @ 2.50GHz, 4 physical/8 logical cores

This was running Slack and all the other usual stuff at the same time so it's not a particularly scientific benchmark.

#### Single-threaded version

```
real    3m49.888s
user    3m35.873s
sys     0m1.021s
```

### Concurrent version
```
real    1m48.583s
user    11m12.174s
sys     0m7.286s
```

### Linux machine, AMD Ryzen 7 5700G, 3.8-4.5GHz, 8 physical/16 logical cores

I've found this might be a bit prone to deadlock. Debugging that is a work in progress. When it runs, it really goes.

#### Single-threaded
```
real    0m33.990s
user    0m33.790s
sys     0m0.200s
```

#### Concurrent
```
real    0m13.793s
user    3m40.330s
sys     0m0.189s
```
