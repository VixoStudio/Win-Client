# Cargo dependency/license inventory

Generated from `cargo metadata --format-version 1 --locked` at audit time.
This is the complete locked Cargo package graph returned by Cargo, including target-specific packages and path/git dependencies for the distributable `pomme-client` package. `Direct` means a dependency declared by `pomme-client`; `transitive` means resolved below those dependencies; `workspace member` identifies local packages in the repository. This inventory is not a legal substitute for reviewing each package license file; `UNSPECIFIED` and compound expressions require manual review before release.

- Package records: **938**
- Root package: `pomme-client`
- Direct dependencies of root: **69**
- Reachable transitive closure: **742**
- Workspace members: **14**
- Resolution source: `Cargo.lock`, workspace manifests, and Cargo patch tables
- Upstream client revision: `ea7e35f1bb647e3327593e708eb6cef12b3035fc`
- SteelMC submodule revision: `e655880a7b1487b2444c70013c14ee86c69d53b3`

| Package | Version | Relation to `pomme-client` | License metadata | Source / provenance |
|---|---:|---|---|---|
| `ab_glyph` | `0.2.32` | transitive | Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `ab_glyph_rasterizer` | `0.1.10` | transitive | Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `adler2` | `2.0.1` | transitive | 0BSD OR MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `aes` | `0.9.3` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `ahash` | `0.8.12` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `aho-corasick` | `1.1.4` | transitive | Unlicense OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `aligned` | `0.4.3` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `aligned-vec` | `0.6.4` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `alloc-no-stdlib` | `3.0.0` | resolved target-specific | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `alloc-stdlib` | `0.3.0` | resolved target-specific | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `allocator-api2` | `0.2.21` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `android-activity` | `0.6.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `android-properties` | `0.2.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `android_system_properties` | `0.1.5` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `anstream` | `1.0.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `anstyle` | `1.0.14` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `anstyle-parse` | `1.0.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `anstyle-query` | `1.1.5` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `anstyle-wincon` | `3.0.11` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `anyhow` | `1.0.102` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `arbitrary` | `1.4.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `arboard` | `3.7.0` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `arc-swap` | `1.9.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `arg_enum_proc_macro` | `0.3.4` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `arrayref` | `0.3.9` | transitive | BSD-2-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `arrayvec` | `0.7.6` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `as-raw-xcb-connection` | `1.0.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `as-slice` | `0.2.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `ascii-canvas` | `4.0.0` | transitive | Apache-2.0/MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `async-broadcast` | `0.7.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `async-channel` | `2.5.0` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `async-executor` | `1.14.0` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `async-io` | `2.6.0` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `async-lock` | `3.4.2` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `async-process` | `2.5.0` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `async-recursion` | `1.1.1` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `async-signal` | `0.2.14` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `async-task` | `4.7.1` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `async-trait` | `0.1.89` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `atk` | `0.18.2` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `atk-sys` | `0.18.2` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `atomic-waker` | `1.1.2` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `autocfg` | `1.5.0` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `av-scenechange` | `0.14.1` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `av1-grain` | `0.2.5` | transitive | BSD-2-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `avif-serialize` | `0.8.9` | transitive | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `aws-lc-rs` | `1.17.0` | transitive | ISC AND (Apache-2.0 OR ISC) | `registry+https://github.com/rust-lang/crates.io-index` |
| `aws-lc-sys` | `0.41.0` | transitive | ISC AND (Apache-2.0 OR ISC) AND Apache-2.0 AND MIT AND BSD-3-Clause AND (Apache-2.0 OR ISC OR MIT) AND (Apache-2.0 OR ISC OR MIT-0) | `registry+https://github.com/rust-lang/crates.io-index` |
| `azalea-auth` | `0.16.0+mc26.2` | direct | MIT | `git+https://github.com/azalea-rs/azalea?rev=ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32#ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32` |
| `azalea-block` | `0.16.0+mc26.2` | direct | MIT | `path: pomme-block/Cargo.toml` |
| `azalea-brigadier` | `0.16.0+mc26.2` | transitive | MIT | `git+https://github.com/azalea-rs/azalea?rev=ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32#ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32` |
| `azalea-buf` | `0.16.0+mc26.2` | direct | MIT | `git+https://github.com/azalea-rs/azalea?rev=ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32#ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32` |
| `azalea-buf-macros` | `0.16.0+mc26.2` | transitive | MIT | `git+https://github.com/azalea-rs/azalea?rev=ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32#ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32` |
| `azalea-chat` | `0.16.0+mc26.2` | direct | MIT | `git+https://github.com/azalea-rs/azalea?rev=ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32#ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32` |
| `azalea-core` | `0.16.0+mc26.2` | direct | MIT | `git+https://github.com/azalea-rs/azalea?rev=ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32#ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32` |
| `azalea-crypto` | `0.16.0+mc26.2` | direct | MIT | `git+https://github.com/azalea-rs/azalea?rev=ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32#ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32` |
| `azalea-entity` | `0.16.0+mc26.2` | direct | MIT | `git+https://github.com/azalea-rs/azalea?rev=ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32#ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32` |
| `azalea-inventory` | `0.16.0+mc26.2` | direct | MIT | `git+https://github.com/azalea-rs/azalea?rev=ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32#ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32` |
| `azalea-inventory-macros` | `0.16.0+mc26.2` | transitive | MIT | `git+https://github.com/azalea-rs/azalea?rev=ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32#ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32` |
| `azalea-language` | `0.16.0+mc26.2` | transitive | MIT | `git+https://github.com/azalea-rs/azalea?rev=ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32#ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32` |
| `azalea-protocol` | `0.16.0+mc26.2` | direct | MIT | `git+https://github.com/azalea-rs/azalea?rev=ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32#ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32` |
| `azalea-protocol-macros` | `0.16.0+mc26.2` | transitive | MIT | `git+https://github.com/azalea-rs/azalea?rev=ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32#ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32` |
| `azalea-registry` | `0.16.0+mc26.2` | direct | MIT | `git+https://github.com/azalea-rs/azalea?rev=ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32#ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32` |
| `azalea-registry-macros` | `0.16.0+mc26.2` | transitive | MIT | `git+https://github.com/azalea-rs/azalea?rev=ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32#ffedf17c9b6ff9fbafbf7e0d14c8d2366b2d0a32` |
| `base16ct` | `1.0.0` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `base64` | `0.21.7` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `base64` | `0.22.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `base64` | `0.23.1` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `base64ct` | `1.8.3` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `bevy_ecs` | `0.18.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `bevy_ecs_macros` | `0.18.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `bevy_macro_utils` | `0.18.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `bevy_platform` | `0.18.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `bevy_ptr` | `0.18.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `bevy_tasks` | `0.18.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `bevy_utils` | `0.18.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `bimap` | `0.6.3` | transitive | Apache-2.0/MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `bit-set` | `0.10.0` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `bit-set` | `0.8.0` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `bit-vec` | `0.8.0` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `bit-vec` | `0.9.1` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `bit_field` | `0.10.3` | transitive | Apache-2.0/MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `bitflags` | `1.3.2` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `bitflags` | `2.13.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `bitstream-io` | `4.10.0` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `block-buffer` | `0.10.4` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `block-buffer` | `0.12.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `block2` | `0.5.1` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `block2` | `0.6.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `blockgen` | `0.1.0` | workspace member | GPL-3.0-or-later | `path: tools/blockgen/Cargo.toml` |
| `blocking` | `1.6.2` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `borsh` | `1.8.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `brotli` | `9.0.0` | resolved target-specific | BSD-3-Clause AND MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `brotli-decompressor` | `6.0.1` | resolved target-specific | BSD-3-Clause/MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `bs58` | `0.5.1` | resolved target-specific | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `built` | `0.8.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `bumpalo` | `3.20.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `bytemuck` | `1.25.2` | direct | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `bytemuck_derive` | `1.10.2` | transitive | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `byteorder` | `1.5.0` | transitive | Unlicense OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `byteorder-lite` | `0.1.0` | transitive | Unlicense OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `bytes` | `1.11.1` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `cairo-rs` | `0.18.5` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `cairo-sys-rs` | `0.18.2` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `calloop` | `0.13.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `calloop-wayland-source` | `0.3.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `camino` | `1.2.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `cargo-platform` | `0.1.9` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `cargo_metadata` | `0.19.2` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `cargo_toml` | `1.0.1` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `castaway` | `0.2.4` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `cbitset` | `0.2.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `cc` | `1.2.62` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `cesu8` | `1.1.0` | resolved target-specific | Apache-2.0/MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `cfb` | `0.14.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `cfb8` | `0.9.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `cfg-expr` | `0.15.8` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `cfg-if` | `1.0.4` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `cfg_aliases` | `0.2.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `chacha20` | `0.10.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `chrono` | `0.4.45` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `cipher` | `0.5.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `clap` | `4.6.7` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `clap_builder` | `4.6.7` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `clap_derive` | `4.6.7` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `clap_lex` | `1.1.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `clipboard-win` | `5.4.1` | transitive | BSL-1.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `cmake` | `0.1.58` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `cmov` | `0.5.3` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `codespan-reporting` | `0.13.1` | transitive | Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `color_quant` | `1.1.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `colorchoice` | `1.0.5` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `colored` | `3.1.1` | transitive | MPL-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `combine` | `4.6.7` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `compact_str` | `0.9.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `concurrent-queue` | `2.5.0` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `const-oid` | `0.10.2` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `convert_case` | `0.10.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `cookie` | `0.18.1` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `core-foundation` | `0.10.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `core-foundation` | `0.9.4` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `core-foundation-sys` | `0.8.7` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `core-graphics` | `0.23.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `core-graphics` | `0.25.0` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `core-graphics-types` | `0.1.3` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `core-graphics-types` | `0.2.0` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `core_maths` | `0.1.1` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `countme` | `3.0.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `cpubits` | `0.1.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `cpufeatures` | `0.2.17` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `cpufeatures` | `0.3.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `crc32c` | `0.6.8` | transitive | Apache-2.0/MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `crc32fast` | `1.5.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `critical-section` | `1.2.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `crossbeam` | `0.8.5` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `crossbeam-channel` | `0.5.17` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `crossbeam-deque` | `0.8.6` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `crossbeam-epoch` | `0.9.18` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `crossbeam-queue` | `0.3.12` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `crossbeam-utils` | `0.8.21` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `crunchy` | `0.2.4` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `crypto-bigint` | `0.7.3` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `crypto-common` | `0.1.7` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `crypto-common` | `0.2.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `crypto-primes` | `0.7.0` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `cssparser` | `0.37.0` | resolved target-specific | MPL-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `cssparser-macros` | `0.7.1` | resolved target-specific | MPL-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `ctor` | `1.0.13` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `ctutils` | `0.4.2` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `cty` | `0.2.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `cursor-icon` | `1.2.0` | transitive | MIT OR Apache-2.0 OR Zlib | `registry+https://github.com/rust-lang/crates.io-index` |
| `darling` | `0.20.11` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `darling` | `0.23.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `darling_core` | `0.20.11` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `darling_core` | `0.23.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `darling_macro` | `0.20.11` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `darling_macro` | `0.23.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `data-encoding` | `2.11.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `dbus` | `0.9.11` | transitive | Apache-2.0/MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `dbus-secret-service` | `4.1.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `der` | `0.8.2` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `deranged` | `0.5.8` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `derive_more` | `2.1.1` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `derive_more-impl` | `2.1.1` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `digest` | `0.10.7` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `digest` | `0.11.3` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `directories` | `6.0.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `dirs` | `7.0.0` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `dirs-sys` | `0.5.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `discord-rich-presence` | `1.1.0` | direct | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `dispatch` | `0.2.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `dispatch2` | `0.3.1` | transitive | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `displaydoc` | `0.2.5` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `disqualified` | `1.0.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `dlib` | `0.5.3` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `dlopen2` | `0.8.2` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `dlopen2_derive` | `0.4.3` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `dom_query` | `0.28.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `downcast-rs` | `1.2.1` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `dpi` | `0.1.2` | transitive | Apache-2.0 AND MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `dtoa` | `1.0.11` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `dtoa-short` | `0.3.5` | resolved target-specific | MPL-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `dunce` | `1.0.5` | transitive | CC0-1.0 OR MIT-0 OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `dyn-clone` | `1.0.20` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `either` | `1.15.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `embed-resource` | `3.0.9` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `embed_plist` | `1.2.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `ena` | `0.14.4` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `encoding_rs` | `0.8.35` | transitive | (Apache-2.0 OR MIT) AND BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `endi` | `1.1.1` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `enum-as-inner` | `0.6.1` | resolved target-specific | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `enum-as-inner` | `0.7.0` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `enum_dispatch` | `0.3.13` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `enumflags2` | `0.7.12` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `enumflags2_derive` | `0.7.12` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `equator` | `0.4.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `equator-macro` | `0.4.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `equivalent` | `1.0.2` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `erased-serde` | `0.4.10` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `errno` | `0.3.14` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `error-code` | `3.3.2` | transitive | BSL-1.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `event-listener` | `5.4.1` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `event-listener-strategy` | `0.5.4` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `exr` | `1.74.0` | transitive | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `fastrand` | `2.5.0` | direct | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `fax` | `0.2.7` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `fdeflate` | `0.3.7` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `field-offset` | `0.3.6` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `find-msvc-tools` | `0.1.9` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `fixedbitset` | `0.5.7` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `flate2` | `1.1.10` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `fnv` | `1.0.7` | transitive | Apache-2.0 / MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `foldhash` | `0.1.5` | transitive | Zlib | `registry+https://github.com/rust-lang/crates.io-index` |
| `foldhash` | `0.2.0` | transitive | Zlib | `registry+https://github.com/rust-lang/crates.io-index` |
| `fontdue` | `0.9.5` | direct | MIT OR Apache-2.0 OR Zlib | `registry+https://github.com/rust-lang/crates.io-index` |
| `foreign-types` | `0.5.0` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `foreign-types-macros` | `0.2.3` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `foreign-types-shared` | `0.3.1` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `form_urlencoded` | `1.2.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `fs_extra` | `1.3.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `futures` | `0.3.34` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `futures-channel` | `0.3.34` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `futures-core` | `0.3.34` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `futures-executor` | `0.3.34` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `futures-io` | `0.3.34` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `futures-lite` | `2.6.1` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `futures-macro` | `0.3.34` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `futures-sink` | `0.3.34` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `futures-task` | `0.3.34` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `futures-util` | `0.3.34` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `gdk` | `0.18.2` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `gdk-pixbuf` | `0.18.5` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `gdk-pixbuf-sys` | `0.18.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `gdk-sys` | `0.18.2` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `gdkwayland-sys` | `0.18.2` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `gdkx11` | `0.18.2` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `gdkx11-sys` | `0.18.2` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `generic-array` | `0.14.7` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `gethostname` | `1.1.0` | transitive | Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `getrandom` | `0.2.17` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `getrandom` | `0.3.4` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `getrandom` | `0.4.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `gif` | `0.14.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `gilrs` | `0.11.2` | direct | Apache-2.0/MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `gilrs-core` | `0.6.8` | transitive | Apache-2.0/MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `gio` | `0.18.4` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `gio-sys` | `0.18.1` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `glam` | `0.34.1` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `glib` | `0.18.5` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `glib-macros` | `0.18.5` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `glib-sys` | `0.18.1` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `glob` | `0.3.4` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `glsl-lang` | `0.8.1` | direct | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `glsl-lang-lexer` | `0.8.1` | transitive | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `glsl-lang-pp` | `0.8.1` | transitive | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `glsl-lang-types` | `0.8.1` | transitive | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `gobject-sys` | `0.18.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `gtk` | `0.18.2` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `gtk-sys` | `0.18.2` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `gtk3-macros` | `0.18.2` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `h2` | `0.4.14` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `half` | `2.7.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `hash32` | `0.3.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `hashbrown` | `0.12.3` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `hashbrown` | `0.14.5` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `hashbrown` | `0.15.5` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `hashbrown` | `0.16.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `hashbrown` | `0.17.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `heapless` | `0.9.3` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `heck` | `0.4.1` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `heck` | `0.5.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `hermit-abi` | `0.5.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `hex` | `0.4.3` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `hickory-net` | `0.26.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `hickory-proto` | `0.25.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `hickory-proto` | `0.26.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `hickory-resolver` | `0.25.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `hickory-resolver` | `0.26.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `html5ever` | `0.39.0` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `http` | `1.4.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `http-body` | `1.0.1` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `http-body-util` | `0.1.3` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `httparse` | `1.10.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `hybrid-array` | `0.4.12` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `hyper` | `1.9.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `hyper-rustls` | `0.27.9` | transitive | Apache-2.0 OR ISC OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `hyper-util` | `0.1.20` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `iana-time-zone` | `0.1.65` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `iana-time-zone-haiku` | `0.1.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `ico` | `0.5.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `icu_collections` | `2.2.0` | transitive | Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `icu_locale_core` | `2.2.0` | transitive | Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `icu_normalizer` | `2.2.0` | transitive | Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `icu_normalizer_data` | `2.2.0` | transitive | Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `icu_properties` | `2.2.0` | transitive | Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `icu_properties_data` | `2.2.0` | transitive | Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `icu_provider` | `2.2.0` | transitive | Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `id-arena` | `2.3.0` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `ident_case` | `1.0.1` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `idna` | `1.1.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `idna_adapter` | `1.2.2` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `image` | `0.25.10` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `image-webp` | `0.2.4` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `imgref` | `1.12.1` | transitive | CC0-1.0 OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `indexmap` | `1.9.3` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `indexmap` | `2.14.0` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `infer` | `0.22.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `Inflector` | `0.11.4` | transitive | BSD-2-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `ingame-test` | `0.1.0` | workspace member | GPL-3.0-or-later | `path: tools/ingame-test/Cargo.toml` |
| `inotify` | `0.11.2` | transitive | ISC | `registry+https://github.com/rust-lang/crates.io-index` |
| `inotify-sys` | `0.1.5` | transitive | ISC | `registry+https://github.com/rust-lang/crates.io-index` |
| `inout` | `0.2.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `interpolate_name` | `0.2.4` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `ipconfig` | `0.3.4` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `ipnet` | `2.12.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `is-docker` | `0.2.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `is-wsl` | `0.4.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `is_terminal_polyfill` | `1.70.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `itertools` | `0.14.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `itoa` | `1.0.18` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `javascriptcore-rs` | `1.1.2` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `javascriptcore-rs-sys` | `1.1.1` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `jni` | `0.21.1` | resolved target-specific | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `jni` | `0.22.4` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `jni-macros` | `0.22.4` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `jni-sys` | `0.3.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `jni-sys` | `0.4.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `jni-sys-macros` | `0.4.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `jobserver` | `0.1.34` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `js-sys` | `0.3.98` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `json-patch` | `4.2.0` | resolved target-specific | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `jsonptr` | `0.7.1` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `keccak` | `0.1.6` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `keyboard-types` | `0.8.3` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `keyring` | `3.6.3` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `lalrpop` | `0.22.2` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `lalrpop-util` | `0.22.2` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `lang-util` | `0.8.1` | transitive | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `lang-util-derive` | `0.8.1` | transitive | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `lazy_static` | `1.5.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `leb128fmt` | `0.1.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `lebe` | `0.5.3` | transitive | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `libappindicator` | `0.9.0` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `libappindicator-sys` | `0.9.0` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `libc` | `0.2.186` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `libdbus-sys` | `0.2.7` | transitive | Apache-2.0/MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `libfuzzer-sys` | `0.4.12` | transitive | (MIT OR Apache-2.0) AND NCSA | `registry+https://github.com/rust-lang/crates.io-index` |
| `libloading` | `0.7.4` | resolved target-specific | ISC | `registry+https://github.com/rust-lang/crates.io-index` |
| `libloading` | `0.8.9` | transitive | ISC | `registry+https://github.com/rust-lang/crates.io-index` |
| `libloading` | `0.9.0` | direct | ISC | `registry+https://github.com/rust-lang/crates.io-index` |
| `libm` | `0.2.16` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `libmimalloc-sys` | `0.1.49` | direct | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `libredox` | `0.1.16` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `libudev-sys` | `0.1.4` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `line-span` | `0.1.5` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `link-cplusplus` | `1.0.12` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `linux-raw-sys` | `0.12.1` | transitive | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `linux-raw-sys` | `0.4.15` | transitive | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `litemap` | `0.8.2` | transitive | Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `lock_api` | `0.4.14` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `log` | `0.4.34` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `loop9` | `0.1.5` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `lru-slab` | `0.1.2` | transitive | MIT OR Apache-2.0 OR Zlib | `registry+https://github.com/rust-lang/crates.io-index` |
| `markup5ever` | `0.39.0` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `matchers` | `0.2.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `maybe-rayon` | `0.1.1` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `md-5` | `0.10.6` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `md-5` | `0.11.0` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `md5` | `0.8.1` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `memchr` | `2.8.0` | transitive | Unlicense OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `memmap2` | `0.9.10` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `memoffset` | `0.9.1` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `mimalloc` | `0.1.52` | direct | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `mime` | `0.3.17` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `miniz_oxide` | `0.8.9` | transitive | MIT OR Zlib OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `miniz_oxide` | `0.9.1` | transitive | MIT OR Zlib OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `mio` | `1.2.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `moka` | `0.12.15` | transitive | (MIT OR Apache-2.0) AND Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `moxcms` | `0.8.1` | transitive | BSD-3-Clause OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `muda` | `0.20.0` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `naga` | `30.0.1` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `naga-types` | `30.0.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `ndk` | `0.9.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `ndk-context` | `0.1.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `ndk-sys` | `0.5.0+25.2.9519653` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `ndk-sys` | `0.6.0+11769913` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `new_debug_unreachable` | `1.0.6` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `nix` | `0.31.3` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `no_std_io2` | `0.9.4` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `nohash-hasher` | `0.2.0` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `nom` | `8.0.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `nonmax` | `0.5.5` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `noop_proc_macro` | `0.3.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `nu-ansi-term` | `0.50.3` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `num` | `0.4.3` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `num-bigint` | `0.4.6` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `num-complex` | `0.4.6` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `num-conv` | `0.2.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `num-derive` | `0.4.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `num-integer` | `0.1.46` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `num-iter` | `0.1.45` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `num-rational` | `0.4.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `num-traits` | `0.2.19` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `num_enum` | `0.7.6` | transitive | BSD-3-Clause OR MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `num_enum_derive` | `0.7.6` | transitive | BSD-3-Clause OR MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `num_threads` | `0.1.7` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc-sys` | `0.3.5` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2` | `0.5.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2` | `0.6.5` | direct | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-app-kit` | `0.2.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-app-kit` | `0.3.2` | transitive | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-cloud-kit` | `0.2.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-cloud-kit` | `0.3.2` | transitive | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-contacts` | `0.2.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-core-data` | `0.2.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-core-data` | `0.3.2` | transitive | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-core-foundation` | `0.3.2` | transitive | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-core-graphics` | `0.3.2` | transitive | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-core-image` | `0.2.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-core-image` | `0.3.2` | transitive | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-core-location` | `0.2.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-core-location` | `0.3.2` | transitive | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-core-text` | `0.3.2` | transitive | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-encode` | `4.1.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-exception-helper` | `0.1.1` | transitive | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-foundation` | `0.2.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-foundation` | `0.3.2` | direct | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-io-kit` | `0.3.2` | transitive | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-io-surface` | `0.3.2` | transitive | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-link-presentation` | `0.2.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-metal` | `0.2.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-metal` | `0.3.2` | transitive | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-quartz-core` | `0.2.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-quartz-core` | `0.3.2` | transitive | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-symbols` | `0.2.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-ui-kit` | `0.2.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-ui-kit` | `0.3.2` | direct | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-uniform-type-identifiers` | `0.2.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-user-notifications` | `0.2.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-user-notifications` | `0.3.2` | transitive | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `objc2-web-kit` | `0.3.2` | resolved target-specific | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `once_cell` | `1.21.4` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `once_cell_polyfill` | `1.70.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `open` | `5.4.4` | direct | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `openssl-probe` | `0.2.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `option-ext` | `0.2.0` | transitive | MPL-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `orbclient` | `0.3.54` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `ordered-stream` | `0.2.0` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `os_pipe` | `1.2.3` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `owned_ttf_parser` | `0.25.1` | transitive | Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `pango` | `0.18.3` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `pango-sys` | `0.18.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `paranoid-android` | `0.2.2` | direct | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `parking` | `2.2.1` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `parking_lot` | `0.12.5` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `parking_lot_core` | `0.9.12` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `paste` | `1.0.15` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `pastey` | `0.1.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `pastey` | `0.2.3` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `pem-rfc7468` | `1.0.0` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `percent-encoding` | `2.3.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `petgraph` | `0.7.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `petgraph` | `0.8.3` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `phf` | `0.11.3` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `phf` | `0.13.1` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `phf` | `0.14.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `phf_codegen` | `0.11.3` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `phf_codegen` | `0.13.1` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `phf_codegen` | `0.14.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `phf_generator` | `0.11.3` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `phf_generator` | `0.13.1` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `phf_generator` | `0.14.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `phf_macros` | `0.13.1` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `phf_macros` | `0.14.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `phf_shared` | `0.11.3` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `phf_shared` | `0.13.1` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `phf_shared` | `0.14.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `pico-args` | `0.5.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `pin-project` | `1.1.13` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `pin-project-internal` | `1.1.13` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `pin-project-lite` | `0.2.17` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `piper` | `0.2.5` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `pkcs1` | `0.8.0-rc.5` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `pkcs8` | `0.11.0` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `pkg-config` | `0.3.33` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `plain` | `0.2.3` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `plist` | `1.9.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `png` | `0.17.16` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `png` | `0.18.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `polling` | `3.11.0` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `pomme-android` | `0.1.0` | workspace member | GPL-3.0-or-later | `path: pomme-android/Cargo.toml` |
| `pomme-auth` | `0.1.0` | direct | GPL-3.0-or-later | `path: pomme-auth/Cargo.toml` |
| `pomme-client` | `0.1.0` | workspace member | GPL-3.0-or-later | `path: pomme-client/Cargo.toml` |
| `pomme-dirs` | `0.1.0` | direct | GPL-3.0-or-later | `path: pomme-dirs/Cargo.toml` |
| `pomme-gpu-allocator` | `0.1.0` | direct | MIT | `path: pomme-gpu-allocator/Cargo.toml` |
| `pomme-install` | `0.1.0` | direct | GPL-3.0-or-later | `path: pomme-install/Cargo.toml` |
| `pomme-ios` | `0.1.0` | workspace member | GPL-3.0-or-later | `path: pomme-ios/Cargo.toml` |
| `pomme-launcher` | `0.1.2` | workspace member | UNSPECIFIED | `path: pomme-launcher/src-tauri/Cargo.toml` |
| `pomme-protocol` | `0.1.0` | direct | GPL-3.0-or-later | `path: pomme-protocol/Cargo.toml` |
| `pomme-singleplayer` | `0.1.0` | direct | AGPL-3.0-or-later | `path: pomme-singleplayer/Cargo.toml` |
| `portable-atomic` | `1.13.1` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `portable-atomic-util` | `0.2.7` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `potential_utf` | `0.1.5` | transitive | Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `powerfmt` | `0.2.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `pp-rs` | `0.2.1` | transitive | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `ppv-lite86` | `0.2.21` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `precomputed-hash` | `0.1.1` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `prefix-trie` | `0.8.4` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `presser` | `0.3.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `prettyplease` | `0.2.37` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `proc-macro-crate` | `1.3.1` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `proc-macro-crate` | `2.0.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `proc-macro-crate` | `3.5.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `proc-macro-error` | `1.0.4` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `proc-macro-error-attr` | `1.0.4` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `proc-macro2` | `1.0.107` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `profiling` | `1.0.18` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `profiling-procmacros` | `1.0.18` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `protogen` | `0.1.0` | workspace member | GPL-3.0-or-later | `path: tools/protogen/Cargo.toml` |
| `pxfm` | `0.1.29` | transitive | BSD-3-Clause OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `pyronyx` | `0.5.0+1.4.357` | direct | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `qoi` | `0.4.1` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `quick-error` | `2.0.1` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `quick-xml` | `0.39.4` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `quinn` | `0.11.9` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `quinn-proto` | `0.11.14` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `quinn-udp` | `0.5.14` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `quote` | `1.0.47` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `r-efi` | `5.3.0` | transitive | MIT OR Apache-2.0 OR LGPL-2.1-or-later | `registry+https://github.com/rust-lang/crates.io-index` |
| `r-efi` | `6.0.0` | transitive | MIT OR Apache-2.0 OR LGPL-2.1-or-later | `registry+https://github.com/rust-lang/crates.io-index` |
| `rand` | `0.10.3` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `rand` | `0.8.6` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `rand` | `0.9.5` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `rand_chacha` | `0.3.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `rand_chacha` | `0.9.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `rand_core` | `0.10.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `rand_core` | `0.6.4` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `rand_core` | `0.9.5` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `rapidhash` | `4.5.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `rav1e` | `0.8.1` | transitive | BSD-2-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `ravif` | `0.13.0` | transitive | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `raw-window-handle` | `0.6.2` | direct | MIT OR Apache-2.0 OR Zlib | `registry+https://github.com/rust-lang/crates.io-index` |
| `raw-window-metal` | `1.1.0` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `rayon` | `1.12.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `rayon-core` | `1.13.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `redox_syscall` | `0.4.1` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `redox_syscall` | `0.5.18` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `redox_syscall` | `0.7.5` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `redox_users` | `0.5.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `ref-cast` | `1.0.25` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `ref-cast-impl` | `1.0.25` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `regex` | `1.13.1` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `regex-automata` | `0.4.18` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `regex-syntax` | `0.8.11` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `replace_with` | `0.1.8` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `reqwest` | `0.13.5` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `resolv-conf` | `0.7.6` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `rfd` | `0.16.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `rgb` | `0.8.53` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `ring` | `0.17.14` | transitive | Apache-2.0 AND ISC | `registry+https://github.com/rust-lang/crates.io-index` |
| `rowan` | `0.16.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `roxmltree` | `0.20.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `rsa` | `0.10.0-rc.19` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `rsa_public_encrypt_pkcs1` | `0.4.0` | transitive | ISC | `registry+https://github.com/rust-lang/crates.io-index` |
| `rustc-hash` | `1.1.0` | transitive | Apache-2.0/MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `rustc-hash` | `2.1.3` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `rustc_version` | `0.4.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `rustix` | `0.38.44` | transitive | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `rustix` | `1.1.4` | transitive | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `rustls` | `0.23.40` | transitive | Apache-2.0 OR ISC OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `rustls-native-certs` | `0.8.3` | transitive | Apache-2.0 OR ISC OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `rustls-pki-types` | `1.14.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `rustls-platform-verifier` | `0.7.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `rustls-platform-verifier-android` | `0.1.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `rustls-webpki` | `0.103.13` | transitive | ISC | `registry+https://github.com/rust-lang/crates.io-index` |
| `rustversion` | `1.0.22` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `ryu` | `1.0.23` | transitive | Apache-2.0 OR BSL-1.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `saa` | `5.6.0` | transitive | Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `same-file` | `1.0.6` | transitive | Unlicense/MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `scc` | `3.8.8` | transitive | Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `schannel` | `0.1.29` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `schemars` | `0.8.22` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `schemars` | `0.9.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `schemars` | `1.2.1` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `schemars_derive` | `0.8.22` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `scoped-tls` | `1.0.1` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `scopeguard` | `1.2.0` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `sctk-adwaita` | `0.10.1` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `sdd` | `4.8.10` | transitive | Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `security-framework` | `2.11.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `security-framework` | `3.7.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `security-framework-sys` | `2.17.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `selectors` | `0.38.0` | resolved target-specific | MPL-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `semver` | `1.0.28` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `serde` | `1.0.229` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `serde-untagged` | `0.1.9` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `serde_core` | `1.0.229` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `serde_derive` | `1.0.229` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `serde_derive_internals` | `0.29.1` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `serde_json` | `1.0.154` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `serde_repr` | `0.1.20` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `serde_spanned` | `0.6.9` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `serde_spanned` | `1.1.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `serde_urlencoded` | `0.7.1` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `serde_with` | `3.20.0` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `serde_with_macros` | `3.20.0` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `serdect` | `0.4.3` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `serialize-to-javascript` | `0.1.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `serialize-to-javascript-impl` | `0.1.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `servo_arc` | `0.4.3` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `sha1` | `0.11.0` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `sha1_smol` | `1.0.1` | direct | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `sha2` | `0.10.9` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `sha2` | `0.11.1` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `sha3` | `0.10.9` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `shaderc` | `0.10.1` | direct | Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `shaderc-sys` | `0.10.1` | transitive | Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `sharded-slab` | `0.1.7` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `shared_child` | `1.1.1` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `shlex` | `1.3.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `signal-hook-registry` | `1.4.8` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `signature` | `3.0.0` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `simd-adler32` | `0.3.9` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `simd_cesu8` | `1.1.1` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `simd_helpers` | `0.1.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `simdnbt` | `0.10.0` | direct | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `simdnbt-derive` | `0.10.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `simdutf8` | `0.1.5` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `simple_asn1` | `0.5.4` | transitive | ISC | `registry+https://github.com/rust-lang/crates.io-index` |
| `siphasher` | `1.0.3` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `slab` | `0.4.12` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `slotmap` | `1.1.1` | transitive | Zlib | `registry+https://github.com/rust-lang/crates.io-index` |
| `small-map` | `0.1.5` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `smallvec` | `1.16.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `smithay-client-toolkit` | `0.19.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `smol_str` | `0.2.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `smol_str` | `0.3.7` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `socket2` | `0.6.3` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `socks5-impl` | `0.8.2` | transitive | GPL-3.0-or-later | `registry+https://github.com/rust-lang/crates.io-index` |
| `softbuffer` | `0.4.8` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `soup3` | `0.5.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `soup3-sys` | `0.5.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `specta` | `2.0.0-rc.24` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `specta-macros` | `2.0.0-rc.24` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `specta-serde` | `0.0.11` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `specta-tags` | `0.0.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `specta-typescript` | `0.0.11` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `spin` | `0.10.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `spirv` | `0.4.0+sdk-1.4.341.0` | transitive | Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `spki` | `0.8.1` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `stable_deref_trait` | `1.2.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `static_assertions` | `1.1.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `static_vcruntime` | `3.0.0` | resolved target-specific | MIT OR Apache-2.0 OR Zlib | `registry+https://github.com/rust-lang/crates.io-index` |
| `steel-core` | `0.16.0+mc26.3` | transitive | AGPL-3.0-or-later | `path: third_party/SteelMC/steel-core/Cargo.toml` |
| `steel-crypto` | `0.16.0+mc26.3` | transitive | AGPL-3.0-or-later | `path: third_party/SteelMC/steel-crypto/Cargo.toml` |
| `steel-login` | `0.16.0+mc26.3` | transitive | AGPL-3.0-or-later | `path: third_party/SteelMC/steel-login/Cargo.toml` |
| `steel-macros` | `0.16.0+mc26.3` | transitive | AGPL-3.0-or-later | `path: third_party/SteelMC/steel-macros/Cargo.toml` |
| `steel-math` | `0.16.0+mc26.3` | transitive | AGPL-3.0-or-later | `path: third_party/SteelMC/steel-math/Cargo.toml` |
| `steel-protocol` | `0.16.0+mc26.3` | transitive | AGPL-3.0-or-later | `path: third_party/SteelMC/steel-protocol/Cargo.toml` |
| `steel-registry` | `0.16.0+mc26.3` | transitive | AGPL-3.0-or-later | `path: third_party/SteelMC/steel-registry/Cargo.toml` |
| `steel-utils` | `0.16.0+mc26.3` | transitive | AGPL-3.0-or-later | `path: third_party/SteelMC/steel-utils/Cargo.toml` |
| `steel-worldgen` | `0.16.0+mc26.3` | transitive | AGPL-3.0-or-later | `path: third_party/SteelMC/steel-worldgen/Cargo.toml` |
| `strict-num` | `0.1.1` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `string_cache` | `0.8.9` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `string_cache` | `0.9.0` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `string_cache_codegen` | `0.5.4` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `string_cache_codegen` | `0.6.1` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `strsim` | `0.11.1` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `subtle` | `2.6.1` | transitive | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `supports-hyperlinks` | `3.2.0` | transitive | Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `swift-rs` | `1.0.9` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `symlink` | `0.1.0` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `symphonia` | `0.5.5` | direct | MPL-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `symphonia-codec-vorbis` | `0.5.5` | transitive | MPL-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `symphonia-core` | `0.5.5` | transitive | MPL-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `symphonia-format-ogg` | `0.5.5` | transitive | MPL-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `symphonia-metadata` | `0.5.5` | transitive | MPL-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `symphonia-utils-xiph` | `0.5.5` | transitive | MPL-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `syn` | `1.0.109` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `syn` | `2.0.117` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `syn` | `3.0.9` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `sync_wrapper` | `1.0.2` | transitive | Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `synstructure` | `0.13.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `system-configuration` | `0.7.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `system-configuration-sys` | `0.6.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `system-deps` | `6.2.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `tagptr` | `0.2.0` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `tao` | `0.37.1` | resolved target-specific | Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `tao-macros` | `0.1.4` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `target-lexicon` | `0.12.16` | resolved target-specific | Apache-2.0 WITH LLVM-exception | `registry+https://github.com/rust-lang/crates.io-index` |
| `tauri` | `2.12.2` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tauri-build` | `2.7.3` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tauri-codegen` | `2.7.1` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tauri-macros` | `2.7.1` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tauri-plugin` | `2.7.1` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tauri-plugin-clipboard-manager` | `2.5.0` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tauri-plugin-dialog` | `2.8.3` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tauri-plugin-fs` | `2.7.1` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tauri-plugin-opener` | `2.7.2` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tauri-plugin-shell` | `2.5.0` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tauri-runtime` | `2.12.1` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tauri-runtime-wry` | `2.12.1` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tauri-specta` | `2.0.0-rc.24` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tauri-specta-macros` | `2.0.0-rc.24` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tauri-utils` | `2.10.2` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tauri-winres` | `0.3.6` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tempfile` | `3.27.0` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `tendril` | `0.5.0` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `term` | `1.2.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `text-size` | `1.1.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `text_components` | `0.1.7` | transitive | GPL-3.0-or-later | `git+https://github.com/Steel-Foundation/TextComponents.git?rev=04d60d0d9c65ab1cb05154105e184d5904e8990b#04d60d0d9c65ab1cb05154105e184d5904e8990b` |
| `thiserror` | `1.0.69` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `thiserror` | `2.0.21` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `thiserror-impl` | `1.0.69` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `thiserror-impl` | `2.0.21` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `thread_local` | `1.1.9` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `tiff` | `0.11.3` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `time` | `0.3.55` | direct | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `time-core` | `0.1.9` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `time-macros` | `0.2.32` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `tiny-skia` | `0.11.4` | transitive | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `tiny-skia-path` | `0.11.4` | transitive | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `tinystr` | `0.8.3` | transitive | Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `tinyvec` | `1.11.0` | transitive | Zlib OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tinyvec_macros` | `0.1.1` | transitive | MIT OR Apache-2.0 OR Zlib | `registry+https://github.com/rust-lang/crates.io-index` |
| `tokio` | `1.53.3` | direct | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tokio-macros` | `2.7.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tokio-rustls` | `0.26.4` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `tokio-util` | `0.7.20` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `toml` | `0.8.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `toml` | `1.1.8+spec-1.1.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `toml_datetime` | `0.6.3` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `toml_datetime` | `0.7.5+spec-1.1.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `toml_datetime` | `1.1.2+spec-1.1.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `toml_edit` | `0.19.15` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `toml_edit` | `0.20.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `toml_edit` | `0.23.10+spec-1.0.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `toml_edit` | `0.25.11+spec-1.1.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `toml_parser` | `1.1.5+spec-1.1.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `toml_writer` | `1.1.3+spec-1.1.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `tower` | `0.5.3` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tower-http` | `0.6.10` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tower-layer` | `0.3.3` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tower-service` | `0.3.3` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tracing` | `0.1.44` | direct | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tracing-appender` | `0.2.5` | direct | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tracing-attributes` | `0.1.31` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tracing-core` | `0.1.36` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tracing-log` | `0.2.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tracing-subscriber` | `0.3.23` | direct | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `tray-icon` | `0.25.1` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `tree_magic_mini` | `3.2.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `try-lock` | `0.2.5` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `ttf-parser` | `0.25.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `typed-path` | `0.12.3` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `typeid` | `1.0.3` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `typenum` | `1.20.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `uds_windows` | `1.2.1` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `unicode-ident` | `1.0.24` | transitive | (MIT OR Apache-2.0) AND Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `unicode-segmentation` | `1.13.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `unicode-width` | `0.2.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `unicode-xid` | `0.2.6` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `unicode_names2` | `4.0.0` | transitive | (MIT OR Apache-2.0) AND Unicode-DFS-2016 | `registry+https://github.com/rust-lang/crates.io-index` |
| `unicode_names2_generator` | `4.0.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `untrusted` | `0.9.0` | transitive | ISC | `registry+https://github.com/rust-lang/crates.io-index` |
| `url` | `2.5.8` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `urlencoding` | `2.1.3` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `urlpattern` | `0.6.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `utf-8` | `0.7.6` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `utf8_iter` | `1.0.4` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `utf8parse` | `0.2.2` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `uuid` | `0.8.2` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `uuid` | `1.28.0` | direct | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `v_frame` | `0.3.9` | transitive | BSD-2-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `valuable` | `0.1.1` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `variadics_please` | `1.1.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `vec_map` | `0.8.2` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `version-compare` | `0.2.1` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `version_check` | `0.9.5` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `vswhom` | `0.1.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `vswhom-sys` | `0.1.3` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `walkdir` | `2.5.0` | transitive | Unlicense/MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `want` | `0.3.1` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wasi` | `0.11.1+wasi-snapshot-preview1` | transitive | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wasip2` | `1.0.3+wasi-0.2.9` | transitive | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wasip3` | `0.4.0+wasi-0.3.0-rc-2026-01-06` | transitive | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wasm-bindgen` | `0.2.121` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `wasm-bindgen-futures` | `0.4.71` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `wasm-bindgen-macro` | `0.2.121` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `wasm-bindgen-macro-support` | `0.2.121` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `wasm-bindgen-shared` | `0.2.121` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `wasm-encoder` | `0.244.0` | transitive | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wasm-metadata` | `0.244.0` | transitive | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wasm-streams` | `0.5.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `wasmparser` | `0.244.0` | transitive | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wayland-backend` | `0.3.15` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wayland-client` | `0.31.14` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wayland-csd-frame` | `0.3.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wayland-cursor` | `0.31.14` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wayland-protocols` | `0.32.12` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wayland-protocols-plasma` | `0.3.12` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wayland-protocols-wlr` | `0.3.12` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wayland-scanner` | `0.31.10` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wayland-sys` | `0.31.11` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `web-sys` | `0.3.98` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `web-time` | `1.1.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `web_atoms` | `0.2.4` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `webkit2gtk` | `2.0.2` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `webkit2gtk-sys` | `2.0.2` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `webpki-root-certs` | `1.0.7` | transitive | CDLA-Permissive-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `webview2-com` | `0.39.1` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `webview2-com-macros` | `0.8.1` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `webview2-com-sys` | `0.39.1` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `weezl` | `0.1.12` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `widestring` | `1.2.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `winapi` | `0.3.9` | resolved target-specific | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `winapi-i686-pc-windows-gnu` | `0.4.0` | resolved target-specific | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `winapi-util` | `0.1.11` | transitive | Unlicense OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `winapi-x86_64-pc-windows-gnu` | `0.4.0` | resolved target-specific | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `wincode` | `0.6.3` | transitive | Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `wincode-derive` | `0.5.1` | transitive | Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `window-vibrancy` | `0.8.1` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows` | `0.62.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-collections` | `0.3.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-core` | `0.62.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-future` | `0.3.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-implement` | `0.60.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-interface` | `0.59.3` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-link` | `0.2.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-numerics` | `0.3.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-registry` | `0.6.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-result` | `0.4.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-strings` | `0.5.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-sys` | `0.45.0` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-sys` | `0.52.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-sys` | `0.59.0` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-sys` | `0.60.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-sys` | `0.61.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-targets` | `0.42.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-targets` | `0.52.6` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-targets` | `0.53.5` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-threading` | `0.2.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows-version` | `0.1.7` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_aarch64_gnullvm` | `0.42.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_aarch64_gnullvm` | `0.52.6` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_aarch64_gnullvm` | `0.53.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_aarch64_msvc` | `0.42.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_aarch64_msvc` | `0.52.6` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_aarch64_msvc` | `0.53.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_i686_gnu` | `0.42.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_i686_gnu` | `0.52.6` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_i686_gnu` | `0.53.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_i686_gnullvm` | `0.52.6` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_i686_gnullvm` | `0.53.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_i686_msvc` | `0.42.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_i686_msvc` | `0.52.6` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_i686_msvc` | `0.53.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_x86_64_gnu` | `0.42.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_x86_64_gnu` | `0.52.6` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_x86_64_gnu` | `0.53.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_x86_64_gnullvm` | `0.42.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_x86_64_gnullvm` | `0.52.6` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_x86_64_gnullvm` | `0.53.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_x86_64_msvc` | `0.42.2` | resolved target-specific | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_x86_64_msvc` | `0.52.6` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `windows_x86_64_msvc` | `0.53.1` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `winit` | `0.30.13` | direct | Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `winnow` | `0.5.40` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `winnow` | `0.7.15` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `winnow` | `1.0.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `winreg` | `0.55.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `winresource` | `0.1.34` | direct | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wit-bindgen` | `0.51.0` | transitive | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wit-bindgen` | `0.57.1` | transitive | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wit-bindgen-core` | `0.51.0` | transitive | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wit-bindgen-rust` | `0.51.0` | transitive | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wit-bindgen-rust-macro` | `0.51.0` | transitive | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wit-component` | `0.244.0` | transitive | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wit-parser` | `0.244.0` | transitive | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `wl-clipboard-rs` | `0.9.3` | transitive | MIT/Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `writeable` | `0.6.3` | transitive | Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `wry` | `0.57.0` | resolved target-specific | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `x11` | `2.21.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `x11-dl` | `2.21.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `x11rb` | `0.13.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `x11rb-protocol` | `0.13.2` | transitive | MIT OR Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `xcursor` | `0.3.10` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `xkbcommon-dl` | `0.4.2` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `xkeysym` | `0.2.1` | transitive | MIT OR Apache-2.0 OR Zlib | `registry+https://github.com/rust-lang/crates.io-index` |
| `y4m` | `0.8.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `yoke` | `0.8.2` | transitive | Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `yoke-derive` | `0.8.2` | transitive | Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `zbus` | `5.15.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `zbus_macros` | `5.15.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `zbus_names` | `4.3.2` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `zerocopy` | `0.8.48` | transitive | BSD-2-Clause OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `zerocopy-derive` | `0.8.48` | transitive | BSD-2-Clause OR Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `zerofrom` | `0.1.8` | transitive | Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `zerofrom-derive` | `0.1.7` | transitive | Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `zeroize` | `1.8.2` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `zeroize_derive` | `1.4.3` | transitive | Apache-2.0 OR MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `zerotrie` | `0.2.4` | transitive | Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `zerovec` | `0.11.6` | transitive | Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `zerovec-derive` | `0.11.3` | transitive | Unicode-3.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `zip` | `8.6.0` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `zip` | `9.0.5` | direct | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `zlib-rs` | `0.6.3` | transitive | Zlib | `registry+https://github.com/rust-lang/crates.io-index` |
| `zmij` | `1.0.21` | transitive | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `zopfli` | `0.8.3` | transitive | Apache-2.0 | `registry+https://github.com/rust-lang/crates.io-index` |
| `zstd` | `0.14.3` | transitive | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `zstd-safe` | `8.2.0` | transitive | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `zstd-sys` | `2.1.2+zstd.1.5.7` | transitive | BSD-3-Clause | `registry+https://github.com/rust-lang/crates.io-index` |
| `zune-core` | `0.5.1` | transitive | MIT OR Apache-2.0 OR Zlib | `registry+https://github.com/rust-lang/crates.io-index` |
| `zune-inflate` | `0.2.54` | transitive | MIT OR Apache-2.0 OR Zlib | `registry+https://github.com/rust-lang/crates.io-index` |
| `zune-jpeg` | `0.5.15` | transitive | MIT OR Apache-2.0 OR Zlib | `registry+https://github.com/rust-lang/crates.io-index` |
| `zvariant` | `5.11.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `zvariant_derive` | `5.11.0` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
| `zvariant_utils` | `3.3.1` | resolved target-specific | MIT | `registry+https://github.com/rust-lang/crates.io-index` |
