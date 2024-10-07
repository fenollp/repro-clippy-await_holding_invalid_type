# repro-clippy-await_holding_invalid_type

Code c/p from https://rust-lang.github.io/rust-clippy/master/index.html#/await_holding_

```shell
     libcrate HEAD λ \cargo clippy --locked --frozen --offline -- -W clippy::await_holding_invalid_type
    Checking libcrate v0.1.0 (/home/pete/wefwefwef/await_holding/libcrate)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     libcrate HEAD λ echo $?
0
```

```
rustc 1.81.0 (eeb90cda1 2024-09-04)
binary: rustc
commit-hash: eeb90cda1969383f56a2637cbd3037bdf598841c
commit-date: 2024-09-04
host: x86_64-unknown-linux-gnu
release: 1.81.0
LLVM version: 18.1.7
```

```
cargo 1.81.0 (2dbb1af80 2024-08-20)
```

```
clippy 0.1.81 (eeb90cd 2024-09-04)
```
