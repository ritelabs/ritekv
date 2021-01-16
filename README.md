# RiteKV

[![crates.io version][1]][2] [![downloads][3]][4] [![docs.rs docs][5]][6]

RiteKV, a experimental key-value pair storage, trying to synthesize interesting research results in recent years.
The main goal is to provide reasonable performance and moderate reliability.

**Work in progress, you can now think of it as a simple in-memory key-value store and evaluate its API.**

- [Documentation][6]
- [Crates.io][2]

## Usage

```rust
let mut store = MemStore::open();
store.set("beep", "boop").unwrap();
let value = store.get("beep").unwrap();
assert_eq!(value, Some("boop".as_bytes().to_owned()));
Ok(())
```

## Contact

Chojan Shang - [@PsiACE](https://github.com/psiace) - <psiace@outlook.com>

Project Link: [https://github.com/ritedb/ritekv](https://github.com/ritedb/ritekv)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or [http://apache.org/licenses/LICENSE-2.0](http://apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

[1]: https://img.shields.io/crates/v/ritekv.svg?style=flat-square
[2]: https://crates.io/crates/ritekv
[3]: https://img.shields.io/crates/d/ritekv.svg?style=flat-square
[4]: https://crates.io/crates/ritekv
[5]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[6]: https://docs.rs/ritekv
