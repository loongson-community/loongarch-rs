# loongarch-rs

A set of Rust crates for working with
[the LoongArch architecture](https://github.com/loongson/LoongArch-Documentation):

* `loongarch-insn`: Assemble and disassemble LoongArch instruction words,
* `loongarch-opcode-file`: Parse instruction table files in the [loongarch-opcodes] format.

[loongarch-opcodes]: https://github.com/loongson-community/loongarch-opcodes

## License

The `loongarch-rs` libraries, as a whole, are licensed under [CC-BY-4.0]
and one of the following licenses:

* [Apache-2.0]
* [MIT]
* [MulanPSL-2.0]

The CC-BY-4.0 requirement is because we incorporate data from
[the loongarch-opcodes project](https://github.com/loongson-community/loongarch-opcodes),
the data tables portion of which is dual-licensed under either [CC-BY-4.0] or
[木兰开放作品许可协议 署名，第 1 版 (Mulan Open Works License Attribution, Version 1)][MulanOWL-BY-1.0].
However, the MulanOWL is not yet standardized by SPDX, hence not usable in
[Cargo's `license` field](https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields),
so only CC-BY-4.0 is taken as a result.
We intend to allow the MulanOWL license as soon as it gets support in SPDX and
Cargo.

If you only make use of some of the provided crates, then not all licenses
apply.
For now, this means if the built-in list of LoongArch instructions is not
pulled in via the `loongarch-insn` crate, then the CC-BY-4.0 license would
not be relevant.

[Apache-2.0]: https://spdx.org/licenses/Apache-2.0.html
[CC-BY-4.0]: https://spdx.org/licenses/CC-BY-4.0.html
[MIT]: https://spdx.org/licenses/MIT.html
[MulanPSL-2.0]: https://spdx.org/licenses/MulanPSL-2.0.html
[MulanOWL-BY-1.0]: https://license.coscl.org.cn/MulanOWLBYv1
