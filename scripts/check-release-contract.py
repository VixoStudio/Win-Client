#!/usr/bin/env python3
"""Validate the downstream release naming, source, and checksum contract."""
import hashlib
from hashlib import sha256
from pathlib import Path
import json
import os
import re
import tarfile
import zipfile

ROOT = Path(__file__).resolve().parents[1]
SHA256 = re.compile(r"^[0-9a-f]{64}$")
GIT_SHA = re.compile(r"^[0-9a-f]{40}$")


def read(path):
    return (ROOT / path).read_text(encoding="utf-8")


def require(text, needle, label):
    if needle not in text:
        raise SystemExit(f"missing {label}: {needle}")


def digest(path):
    return sha256(path.read_bytes()).hexdigest()


def validate_dist(dist, tag):
    expected = [
        "win-client-windows-x64.zip",
        "win-client-windows-x64-setup.exe",
        "sha256sums.txt",
        f"Win-Client-{tag}-source.tar.gz",
        f"Win-Client-{tag}-manifest.json",
    ]
    missing = [name for name in expected if not (dist / name).is_file()]
    if missing:
        raise SystemExit("release dist missing: " + ", ".join(missing))

    manifest_path = dist / f"Win-Client-{tag}-manifest.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    source = f"Win-Client-{tag}-source.tar.gz"
    if manifest.get("release_tag") != tag or manifest.get("source_asset") != source:
        raise SystemExit("manifest release tag/source asset is inconsistent")
    if not SHA256.fullmatch(manifest.get("source_sha256", "")):
        raise SystemExit("manifest source_sha256 is not a SHA-256 digest")
    if not GIT_SHA.fullmatch(manifest.get("source_commit", "")):
        raise SystemExit("manifest source_commit is incomplete")
    if not GIT_SHA.fullmatch(manifest.get("steelmc_commit", "")):
        raise SystemExit("manifest steelmc_commit is incomplete")

    entries = manifest.get("assets")
    if not isinstance(entries, list) or not entries:
        raise SystemExit("manifest assets must be a non-empty list")
    listed = {entry.get("name"): entry.get("sha256") for entry in entries}
    actual_assets = {p.name for p in dist.iterdir() if p.is_file() and p.name not in {manifest_path.name, "sha256sums.txt"}}
    if set(listed) != actual_assets:
        raise SystemExit(f"manifest assets mismatch: listed={sorted(listed)} actual={sorted(actual_assets)}")
    for name, expected_hash in listed.items():
        if not isinstance(name, str) or not SHA256.fullmatch(expected_hash or ""):
            raise SystemExit(f"manifest has invalid hash for {name!r}")
        actual_hash = digest(dist / name)
        if actual_hash != expected_hash:
            raise SystemExit(f"manifest hash mismatch for {name}: {actual_hash} != {expected_hash}")
    if listed[source] != manifest["source_sha256"]:
        raise SystemExit("manifest source_sha256 does not match assets entry")
    if not SHA256.fullmatch(manifest.get("executable_sha256", "")):
        raise SystemExit("manifest executable_sha256 is not a SHA-256 digest")
    import zipfile
    with zipfile.ZipFile(dist / "win-client-windows-x64.zip") as archive:
        if hashlib.sha256(archive.read("win-client.exe")).hexdigest() != manifest["executable_sha256"]:
            raise SystemExit("manifest executable_sha256 does not match win-client.exe")

    sums = {}
    for line in (dist / "sha256sums.txt").read_text(encoding="utf-8").splitlines():
        digest_text, name = line.split(None, 1)
        sums[Path(name.lstrip("* ")).name] = digest_text
    expected_sums = {entry.name: digest(entry) for entry in dist.iterdir() if entry.is_file() and entry.name != "sha256sums.txt"}
    if sums != expected_sums:
        raise SystemExit("sha256sums.txt does not match release files")

    with tarfile.open(dist / source, "r:gz") as archive:
        names = set(archive.getnames())
        prefix = f"Win-Client-{tag}/third_party/SteelMC/"
        steelmc_files = [name for name in names if name.startswith(prefix)]
        if not steelmc_files:
            raise SystemExit("source archive does not contain SteelMC source")
        marker = prefix + ".git-submodule-commit"
        metadata = prefix + ".git-commit-metadata"
        root_metadata = f"Win-Client-{tag}/.git-commit-metadata"
        if marker not in names or metadata not in names or root_metadata not in names:
            raise SystemExit("source archive is missing root or SteelMC commit metadata")
        marker_value = archive.extractfile(marker).read().decode().strip()
        if marker_value != manifest["steelmc_commit"]:
            raise SystemExit("SteelMC archive marker does not match manifest")
        root_value = archive.extractfile(root_metadata).read().decode().splitlines()[0].strip()
        if root_value != manifest["source_commit"]:
            raise SystemExit("root source metadata does not match manifest")


def main():
    workflow = read(".github/workflows/release-client.yml")
    installer = read("packaging/windows/pomme.iss")
    updater = read("pomme-install/src/client.rs")
    notice = read("pomme-client/third-party/OpenAL-Soft-NOTICE.txt")
    license_text = read("pomme-client/third-party/OpenAL-Soft-LICENSE.txt")
    linux = read("packaging/linux.sh")
    main_screen = read("pomme-client/src/ui/menu/main_screen.rs")
    credits = read("pomme-client/src/ui/menu/credits.rs")
    in_game = read("pomme-client/src/app/phases/in_game.rs")

    require(workflow, "artifact: win-client.exe", "Windows executable artifact")
    require(workflow, "name: win-client-windows-x64", "Windows asset stem")
    require(workflow, "${{ matrix.name }}.zip", "Windows ZIP packaging")
    require(workflow, "${{ matrix.name }}-setup", "Windows installer packaging")
    require(workflow, "WIN_CLIENT_RELEASE: ${{ needs.check.outputs.tag }}", "release environment")
    require(workflow, "Prepare corresponding source archive with SteelMC", "SteelMC source staging")
    require(workflow, ".git-submodule-commit", "SteelMC pinned commit marker")
    require(workflow, ".git-commit-metadata", "SteelMC commit metadata")
    require(workflow, "manifest.json", "release manifest generation")
    require(workflow, "hashlib.sha256", "per-asset SHA-256 manifest generation")
    require(workflow, "Generate checksums after all release assets are ready", "final checksum generation")
    require(workflow, "Validate prepared release contract", "release validation gate")

    require(installer, "AppId={{B7E3A4D1-5C2F-4B9E-8A61-3D0F6C2A9174}", "stable downstream AppId")
    require(installer, "Filename: \"{app}\\win-client.exe\"", "installer launcher executable")
    require(updater, '"win-client-windows-x64.zip"', "Windows updater ZIP")
    require(updater, '"win-client-linux-x64-gnu.zip"', "Linux updater ZIP")
    require(updater, '"win-client-macos-arm64.zip"', "macOS updater ZIP")
    require(linux, 'ln -s usr/bin/win-client "$appdir/AppRun"', "Linux internal binary path")
    for item in ("OpenAL Soft", "1.25.1", "GNU LIBRARY GENERAL PUBLIC LICENSE"):
        require(notice + license_text, item, "OpenAL notice/license entry")

    require(main_screen, 'text_width_fn("WinModing", title_size)', "main-screen downstream wordmark")
    require(main_screen, 'text: "WinModing".into()', "main-screen downstream title")
    require(main_screen, 'format!("WinModing {}", crate::update::version())', "main-screen downstream version copy")
    require(credits, 'https://github.com/vithorixscom-debug/Win-Client/blob/master/THIRD_PARTY_LICENSES.md', "downstream license URL")
    require(credits, 'text_width_fn("WinModing", wordmark_size)', "credits downstream wordmark")
    require(credits, 'text: "WinModing".into()', "credits downstream title")
    require(in_game, 'https://github.com/vithorixscom-debug/Win-Client/issues', "downstream issue URL")
    require(in_game, 'format!("WinModing {}", env!("CARGO_PKG_VERSION"))', "debug version downstream title")
    for source in (main_screen, credits, in_game):
        if "Pomme Debugger" in source or "https://github.com/PommeMC/Client/issues" in source:
            raise SystemExit("legacy Windows-facing Pomme branding remains")

    updater_contract = updater.split("#[cfg(test)]", 1)[0]
    for legacy in ("pomme-client-linux-x64-gnu.zip", "pomme-client-macos-arm64.zip", "pomme-client-linux-x64-gnu.AppImage", "pomme-client-macos-arm64.dmg"):
        if legacy in workflow + updater_contract:
            raise SystemExit(f"upstream platform asset name remains in release contract: {legacy}")

    dist_name = os.environ.get("RELEASE_DIST")
    if dist_name:
        validate_dist(Path(dist_name), os.environ.get("RELEASE_TAG", ""))
    print("release contract: OK")


if __name__ == "__main__":
    main()
