# facet-args-rs-script-demos

Standalone scripts and their equivalent crates for experimenting with facet-args in no-std mode

## Motivation

Using rust-script (https://rust-script.org/) to use Rust as a scripting language to experiment with
facet-args in a single file.

Just execute the script file directly and due to the shebang line it'll get compiled and run. If the
compilation size is small enough, it'll be as fast (ish) as booting up an interpreter in a language
like Python!

The cool thing about this is you can go grab the Cargo.toml stashed in your user cache (`$XDG_CACHE_HOME` or `$HOME/.cache/rust-script/projects/<uuid>`) and copy it under a crates dir (as done here), and edit the source file of the `[[bin]]` target to be relative (`../../std_parser.rs` etc). Funky upside down package structure.

---

For context, this was a quick side investigation while building [unjust-cli](https://github.com/lmmx/unjust)!
