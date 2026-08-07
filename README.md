# Maliit

Maliit is a simple CLI file organizer written in Rust.

**Work in Progress** - Currently, it organizes image files by user-specified extensions into a directory of your choice.

## Usage

Organize all images in the source directory
```
cargo run -- organize ~/src-path ~/dest-path -t image
```

If specifying file extensions
```
cargo run -- organize ~/src-path ~/dest-path -t image -e jpg png
```

## Flags

* `-t, --type` — Filter by file category (`docs`, `image`).
* `-e, --ext` — Filter by file extensions (e.g., `jpg png`).
* `-o, --ovr` — Overwrite duplicate files in the destination directory.