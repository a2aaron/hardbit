# Build Instructions
First, make sure you have [Rust](https://www.rust-lang.org/) installed.

To build the plugin as a vst3 bundle, run the following command:

```
cargo xtask bundle hardbit --release
```

This will create a `hardbit.vst3` bundle in `/target/bundled/`. Install this into any DAW of your choice.


You can also create a standalone binary by running the following command:

```
cargo build --release --bin standalone
```

This will create a `standalone` binary in `/target/release/`. You can see the arguments it uses with `standalone -h`. See [nih-plug](https://github.com/robbert-vdh/nih-plug) for more information.
