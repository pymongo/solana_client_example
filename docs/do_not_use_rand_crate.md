官方文档说 program不能依赖rand <https://solana.com/docs/programs/lang-rust#depending-on-rand>

```
error: target is not supported, for more information see: https://docs.rs/getrandom/#unsupported-targets
   --> src/lib.rs:267:9
    |
267 | /         compile_error!("\
268 | |             target is not supported, for more information see: \
269 | |             https://docs.rs/getrandom/#unsupported-targets\
270 | |         ");
    | |__________^

   Compiling hashbrown v0.13.2
   Compiling rand_chacha v0.3.1
error[E0433]: failed to resolve: use of undeclared crate or module `imp`
   --> src/lib.rs:291:5
    |
291 |     imp::getrandom_inner(dest)
    |     ^^^ use of undeclared crate or module `imp`

   Compiling solana-program v2.0.2
   Compiling quote v1.0.36
For more information about this error, try `rustc --explain E0433`.
error: could not compile `getrandom` (lib) due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
```

cargo build-bpf 在 workspace 中会报错

```
error[E0425]: cannot find value `SOCK_RAW` in module `sys`
   --> src/lib.rs:283:37
    |
283 |     pub const RAW: Type = Type(sys::SOCK_RAW);
    |                                     ^^^^^^^^ not found in `sys`
```

以上两个报错都是因为，cargo build-bpf 不支持在 workspace root 下执行，cd 到 solana_client_example/programs/greeting_account 下面执行就没有报错了
