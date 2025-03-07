# to run
either install rust and use `cargo run --release`
or run the executable that is prebuilt
`./target/release/WBT`

the source code is sortof a mess, sorry about that one.
lines 1-284 are the actual tree algorithm, the rest are just testing/display stuffs


my tree is much slower than the rust one, this is expected, considering rust sort is also `O(log(n))` but much more optimized.

you can see the time graph at https://docs.google.com/spreadsheets/d/1-zjBF5vTbRfMhtp6Yv8JsRea2u2nIg567Varq05w_Ag/edit?usp=sharing

its sorta sketchy because we are timing on the order of microseconds even with 16m elements
