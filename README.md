# chaos-engine-contain
This crate simply makes the process of closing and opening programs being developed with the [chaos-engine](https://crates.io/crate/chaos-engine) crate easier, by containing its
process and automatically building and starting a new one if the process ends with code 1.

To use this crate, simply install it:
```
cargo install chaos-engine-contain
```

Then in your chaos-engine project directory:
```
chaos-contain
```

You have to make your program exit with code 1 if you want it to restart itself.
