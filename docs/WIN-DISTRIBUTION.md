# Windows distribution audit

**Audit scope:** nested `win-client/` source tree. Task 2 applies the downstream identity/release-contract changes listed below; upstream attribution and license files remain unchanged. The audit was performed against the checked-out upstream tree at commit `ea7e35f1bb647e3327593e708eb6cef12b3035fc` (`master`, 2026-10-29) with SteelMC submodule at `e655880a7b1487b2444c70013c14ee86c69d53b3`. Upstream provenance is `https://github.com/PommeMC/Client` (`origin` points to that URL).

## 1. Windows executable and launch contract

- **Executable entry point:** `pomme-client/src/main.rs::main`. Release builds use `windows_subsystem = "windows"`, so no console window is opened. `main` parses `LaunchArgs` with clap and calls `pomme_client::run`.
- **Windows product executable name (downstream contract):** `win-client.exe` (Rust package/library remains `pomme-client`; its explicit `[[bin]]` target is `win-client`). Workflow, installer and launcher discovery must all use this filename. Non-Windows builds retain upstream binary naming.
- **CLI parser identity:** clap command name `win-client`, description `Win Minecraft client` (`pomme-client/src/args.rs`).
- **Arguments:** `--version`; `--width` and `--height` (defaults `854` x `480`, logical pixels); `--username`, `--uuid`, `--access-token`, `--launch-token`; `--assets-dir`, `--versions-dir`, `--game-dir`; mutually exclusive `--quick-access-multiplayer` and `--quick-access-singleplayer`; `--chunk-bench <RD>` and `--headless`. The `cpu-bench`, `mesh-hash`, and test-harness arguments exist only when those non-release features are enabled. The old-launcher `--launch-token` disables the client's own updater.
- **Window identity:** title is `Win Client`; the embedded PNG/ICO remains the unmodified upstream Pomme icon because no WinModing-owned icon provenance/licensing was established. Do not present that icon as Win-owned.
- **Data directory:** by default `%LOCALAPPDATA%\Win Client` on Windows, via `directories::BaseDirs::data_local_dir()`. The existing generic `.pomme` migration still migrates old data on first launch; confirm its per-platform behavior before release. Layout: `minecraft/assets`, `minecraft/versions`, `pomme-assets`, `bedrock/<tag>`, `profiles/<name>` (options, servers, saves, screenshots, logs, mods, packs), `cache/<version>` (rebuildable), and `launcher` state. Explicit `--assets-dir`/`--versions-dir`/`--game-dir` override the corresponding locations; if no asset/version override is supplied, the client owns the default data folder.

## 2. Assets and runtime dependencies

- Minecraft assets are not shipped in the Windows bundle. The client downloads the selected version's assets on first start, into the data directory; an owned version install contains the asset index/objects, version JSON/JAR, and extracted assets.
- Pomme-branded assets are in `pomme-client/assets/` (`icon.png`, `icon.ico`, `icon.icns`) and the PNG is embedded by `src/assets.rs`; the Windows ICO is embedded in the executable and used by the installer.
- Shaders are compiled from `pomme-client/src/renderer/shaders` during the Rust build by `build.rs` using `shaderc`; generated SPIR-V and compressed block data are embedded/generated in Cargo `OUT_DIR`, not separate release files.
- Graphics: Vulkan is required at build time (Vulkan SDK). Windows uses the platform Vulkan loader/runtime; the Windows workflow does not copy MoltenVK or a Vulkan loader. A compatible Vulkan driver must be installed on the user's system.
- Audio: the Windows release stages the unmodified OpenAL Soft native from Minecraft/LWJGL using `tools/fetch_openal.py` and includes `openal32.dll` alongside the executable. `OpenAL-Soft-LICENSE.txt` and `OpenAL-Soft-NOTICE.txt` are included beside it. Without the native, audio is disabled. Audio decoding also uses the Rust `symphonia` crate.
- Other relevant runtime behavior: Microsoft sign-in uses the OS keyring; network access is required for authentication, asset/version downloads and update checks. No JVM is required.

## 3. Build and exact release output

Pinned toolchain is `nightly-2026-08-21` (`rust-toolchain.toml`), with clippy, rustfmt, rust-analyzer and rust-src. A source build requires `just`, Rust/rustup, the Vulkan SDK, Python, and a clone with submodules when the default singleplayer feature is enabled.

Local recipes:

```text
just client-build --release
just client-release [client args]
```

`client-build` builds `pomme-client`; `client-release` runs the optimized binary after staging OpenAL. The release workflow builds target `x86_64-pc-windows-msvc` with default features and `WIN_CLIENT_RELEASE` set to the release tag, then stages OpenAL with `--classifier natives-windows --require`.

The Windows packaging workflow produces exactly:

- portable ZIP: `dist/win-client-windows-x64.zip`
- installer: `dist/win-client-windows-x64-setup.exe`
- ZIP/installer contents: `win-client.exe`, `openal32.dll`, `OpenAL-Soft-LICENSE.txt`, and `OpenAL-Soft-NOTICE.txt`.
- Installer remains `packaging/windows/pomme.iss` (filename is internal/upstream packaging path); AppName is Win Client, install dir is `%LOCALAPPDATA%\Programs\Win Client`, executable is `win-client.exe`. The downstream AppId is `{B7E3A4D1-5C2F-4B9E-8A61-3D0F6C2A9174}`; it is intentionally separate from upstream so install/uninstall state is not conflated. Installer upgrade behavior still requires a Windows smoke test.

The Inno Setup script (`packaging/windows/pomme.iss`) installs the same bundle per-user to `%LOCALAPPDATA%\Programs\Win Client`, creates Start Menu and optional desktop shortcuts, and launches `win-client.exe`. Its AppId is `{B7E3A4D1-5C2F-4B9E-8A61-3D0F6C2A9174}`. The updater downloads the ZIP asset by the hard-coded name above; changing the asset name requires a coordinated updater/workflow change.

The release workflow uploads the Windows ZIP/installer, Linux and macOS platform assets, the corresponding source archive, a complete per-asset SHA-256 manifest, and `sha256sums.txt` to a `client-v*` GitHub release. The manifest is generated only after all platform assets are ready, and the contract check verifies every listed file and digest before publication. Windows release publication is configured by `.github/workflows/release-client.yml`; this audit did not publish anything.

## 4. SteelMC inclusion and source obligations

`pomme-client` defaults to feature `singleplayer`, which enables optional `pomme-singleplayer`; that crate links the `third_party/SteelMC` submodule at the inspected commit. Therefore the normal Windows release **does include SteelMC** in the combined client executable. `cargo build -p pomme-client --release --no-default-features` omits SteelMC and is not the feature-complete singleplayer release.

The upstream README identifies the client as GPL-3.0-or-later, copyright `(C) 2026 Purdze`; `pomme-singleplayer` is AGPL-3.0-or-later because of SteelMC, with GPL section 13 noted in the repository. `pomme-gpu-allocator` and `pomme-block` are MIT. Other recorded notices are in `THIRD_PARTY_LICENSES.md`, including binary-greedy-meshing (MIT), ash (MIT), gpu-allocator (MIT), SteelMC (AGPL-3.0-or-later), OpenAL Soft (LGPL-2.0), macOS-only Vulkan components, AppImage runtime, and azalea (MIT).

For a Windows binary distribution, keep `LICENSE`, `THIRD_PARTY_LICENSES.md`, SteelMC's license/source provenance, and the OpenAL license/notice in the source/release notices. The locked Cargo graph is inventoried in [`WIN-DEPENDENCY-LICENSE-INVENTORY.md`](./WIN-DEPENDENCY-LICENSE-INVENTORY.md), generated with `cargo metadata --format-version 1 --locked`; it records all 938 resolved package records, including path, registry, and pinned git dependencies. Any `UNSPECIFIED` or compound license expression remains a manual release-review item, and the relevant package license files must be included or linked according to their terms.

The corresponding-source contract for every binary release is generated by the workflow as `Win-Client-<release-tag>-source.tar.gz` from the exact release commit. The archive contains the full repository tree plus materialized `third_party/SteelMC` source at the pinned revision, `.git-submodule-commit`, and `.git-commit-metadata`. It is uploaded alongside every platform asset. The workflow then generates `Win-Client-<release-tag>-manifest.json` after all assets exist; its `assets` list contains each release asset name and SHA-256, including the Windows ZIP/installer, Linux and macOS assets, and source archive. `sha256sums.txt` is generated from the same final directory. The contract check requires the SteelMC source/marker and verifies both manifest hashes and checksum-file hashes before publication. A release is invalid if any asset, digest, or exact commit/submodule metadata is missing. The source asset must be retained for the full applicable GPL/AGPL offer period.

No product-code audit in this task established that a legal-notices/About surface is present in the Windows client. Before release, add or verify a convenient, prominent notice path appropriate to the UI and include GPL/AGPL and third-party notices in the release bundle/source publication.

## 5. Provenance, branding, and non-endorsement

- Upstream source/provenance: PommeMC/Client, `https://github.com/PommeMC/Client`, inspected commit `ea7e35f1bb647e3327593e708eb6cef12b3035fc`; original author/copyright identified by the repository as Purdze, copyright 2026. SteelMC source is `https://github.com/Steel-Foundation/SteelMC`, inspected commit `e655880a7b1487b2444c70013c14ee86c69d53b3`.
- The current `pomme-client/assets/icon.*` are upstream assets retained for attribution and licensing reasons; they are not presented as WinModing-owned brand assets. A future icon replacement requires recorded ownership, permission/license, provenance, and exact file SHA-256 values.
- Preserve the upstream notice: **Pomme is not affiliated with or endorsed by Mojang Studios or Microsoft. Minecraft is a trademark of Mojang Studios.** The README now states that this is an independent downstream build and is not affiliated with or endorsed by PommeMC, Mojang Studios, Microsoft, SteelMC, or other upstream licensors. The original Pomme/Mojang notice and Minecraft trademark statement are retained.

## 6. Identity/path/icon change inventory

For a WinModing-branded Windows product, the following product-owned values must be deliberately changed together (or explicitly retained with a documented reason):

- `pomme-client/src/args.rs`: clap command name/help identity (`win-client`).
- `pomme-client/src/app/mod.rs`: window title `Win Client`.
- `pomme-client/src/audio/mod.rs`: diagnostic audio-engine name `Pomme Sound engine`.
- `pomme-client/src/discord.rs`: Discord Rich Presence details/large text.
- `pomme-client/src/app/install.rs`, updater/install code, user-facing errors and issue URLs: downstream Win Client wording and standalone Win-Client issue URL; upstream names remain only where required for technical identifiers or attribution.
- `pomme-client/src/main.rs`/Cargo package and all `pomme-*` crate/package identifiers only if a full technical rename is intended; this is broader than branding and must preserve dependency paths and release compatibility.
- `pomme-client/assets/icon.png`, `icon.ico`, `icon.icns`: replace only with assets whose WinModing provenance and license are recorded. `pomme-client/build.rs` and the Inno Setup `SetupIconFile` consume the ICO.
- `packaging/windows/pomme.iss`: `AppId`, `AppName`, `AppPublisher`, `AppPublisherURL`, install directory, shortcut names, executable name if changed, and output naming. Changing AppId or install/data paths affects upgrade/uninstall behavior and must be an intentional migration.
- `pomme-dirs/src/lib.rs`: Windows data-root leaf `Pomme`; changing it creates a new data namespace and requires an explicit migration plan from `%LOCALAPPDATA%\Pomme`.
- `.github/workflows/release-client.yml` and `pomme-install/src/client.rs`: release repository URL, release API URL, release tag/asset names, and checksum/update contract.
- launcher metadata if shipped: `pomme-launcher/src-tauri/tauri.conf.json` (`Pomme Launcher`, `com.pomme.launcher`) and its icon references.

The following third-party/upstream notices must **not** be silently changed or removed: copyright/license text in `LICENSE`, `THIRD_PARTY_LICENSES.md`, `pomme-client/third-party/OpenAL-Soft-LICENSE.txt`, `pomme-client/third-party/OpenAL-Soft-NOTICE.txt`, SteelMC's license and attribution, azalea and other MIT notices, and the Mojang/Minecraft trademark/non-endorsement language. Any downstream modification notice must be additive and clearly identify the modified date/build.

## 7. Repository and publication gate

Read-only checks performed:

- `gh auth status`: authenticated as `vithorixscom-debug`.
- Existing upstream repository is reachable: `PommeMC/Client`, default branch `master`, URL `https://github.com/PommeMC/Client`.
- `vithorixscom-debug/WinModing` exists at `https://github.com/vithorixscom-debug/WinModing`.
- `vithorixscom-debug/win-client` was not found. This is a publication/target-repository blocker if the intended destination is a separate `win-client` repository; no repository was created and no push/dispatch/release was attempted.

Publication gates before any redistribution change:

1. Use the separate repository `vithorixscom-debug/Win-Client` (or record a documented alternative only if that exact name is unavailable); create it only after local artifact and compliance gates pass.
2. Record WinModing brand asset ownership/licensing and replace all product identity assets/strings consistently.
3. Build the exact default-feature Windows bundle, verify the ZIP contents and OpenAL checksum, and run a clean Windows smoke test including first launch, data migration, sign-in/asset download, audio, Vulkan rendering, updater and uninstall.
4. Let `.github/workflows/release-client.yml` generate the exact `Win-Client-<release-tag>-source.tar.gz` archive and `Win-Client-<release-tag>-manifest.json`; the workflow contract check must pass for every binary publication.
5. Publish the exact matching corresponding source and all notices/licenses, including materialized SteelMC source at the pinned revision and its commit metadata.
6. Confirm release asset names/checksums and downloader URLs agree, verify non-endorsement/trademark wording, and only then create/push a repository or GitHub release.
