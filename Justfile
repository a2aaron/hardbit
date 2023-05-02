hello:
    clang -nostdlib hello.s -l system -o target/hello -g
    ./target/hello

perf name:
    -trash {{name}}.trace
    -trash {{name}}.wav
    cargo build --release
    cargo xtask bundle hardbit --release
    xctrace record --template 'Time Profiler' --output {{name}}.trace --launch -- target/release/perf --in megalovania.mid --out {{name}}.wav --polycat

vplayer:
    -trash hardbit_stdout.log
    -trash hardbit_stderr.log
    -pkill "vPlayer 3"
    cargo xtask bundle hardbit --release
    open /Applications/vPlayer\ 3.app/ --stdout ./hardbit_stdout.log --stderr ./hardbit_stderr.log

run:
    cargo run --release --bin standalone -- --midi-input "USB Axiom 49 Port 1" --backend auto --sample-rate 44100

release-all:
    cargo build --release
    cargo xwin build --release --target x86_64-pc-windows-msvc
    cargo xtask bundle-universal hardbit --release
    strip -Sx target/bundled/hardbit.vst3/Contents/MacOS/hardbit
    codesign -f -s - target/bundled/hardbit.vst3

release-small:
    cargo build --release
    ls -lh target/release/libhardbit.dylib
    strip target/release/libhardbit.dylib -Sx
    ls -lh target/release/libhardbit.dylib