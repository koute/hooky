# Hooky - an LD_PRELOAD hooker

[![Documentation](https://docs.rs/hooky/badge.svg)](https://docs.rs/hooky/*/hooky/)

This crate allows convenient `LD_PRELOAD`-style hooking of functions.

## Getting started

1. Go into `examples/yesterday` and type:

       $ cargo build

2. Check the current date:

       $ /bin/bash -c "date"
       Tue Apr 25 18:19:57 CEST 2017

3. Check the date again:

       $ LD_PRELOAD=./target/debug/libyesterday.so /bin/bash -c "date"
       Mon Apr 24 18:20:25 CEST 2017

It seems like we went back in time!

## License

Licensed under either of

  * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
  * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
