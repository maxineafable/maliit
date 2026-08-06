# Maliit

Maliit is a simple CLI file organizer written in Rust.

**Work in Progress** - Currently, it organizes image files by user-specified extensions into a directory of your choice.

## Usage

Organize all images in the source directory
```
cargo run -- images ~/src-path ~/dest-path
```

If specifying file extensions
```
cargo run -- images ~/src-path ~/dest-path --ext jpg png
```