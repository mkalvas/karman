# `karman`

A distributed system that solves the challenges from [fly.io](https://fly.io/dist-sys). Named after the [`maelstrom`](https://github.com/jepsen-io/maelstrom) tool that the challenges are built on and the term [Kármán vortex streets](https://en.wikipedia.org/wiki/K%C3%A1rm%C3%A1n_vortex_street).

Written in Rust, because Rust is awesome.

./maelstrom/maelstrom test -w echo --time-limit 30 --rate 1000 --bin target/release/karman
