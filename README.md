<br />
<br />
<p align="center">
  <a href="">
    <picture>
       <source media="(prefers-color-scheme: dark)" srcset="assets/tezca_light.svg">
      <source media="(prefers-color-scheme: light)" srcset="assets/tezca_dark.svg">
    <img alt="Trezca Logo" src="assets/tezca_light.svg" height="250"/>
    </picture>
  </a>
</p>
<div align="center">
<br />
<br />
<div align="center">
  <a href="">
    <img alt="" src="https://img.shields.io/badge/any_text-you_like-green">
  </a>
  <a href="">
    <img alt="" src="https://img.shields.io/badge/any_text-you_like-red" height="20" width="auto">
  </a>
  <a href="">
    <img alt="License Apache 2.0" src="https://img.shields.io/badge/license-Apache 2.0-blue.svg?style=flat&color=3b82f6&labelColor=334155 " height="20" width="auto">
  </a>
  <a href="">
    <img src="https://img.shields.io/badge/any_text-you_like-purple" alt="" />
  </a>
  <a href="">
    <img src="https://img.shields.io/badge/any_text-you_like-yellow" alt="" />
  </a>
</div>

<br />
<h1 align="center">Supafast Image Similarity Indexing</h1>
</div>

### Getting Started *(beta)*

Install [rust](https://www.rust-lang.org/tools/install) if you haven't

This will prepare the release binary on your local machine

```bash
git clone https://github.com/udaypatel1/tezca.git
cd tezca
cargo build --release
```

To run the release binary, do the following:

```bash
cargo run --release assets/sf.jpg assets/la.jpg
```

| Index Value   | Meaning                          |
|---------------|----------------------------------|
| 0.99          | Near perfect positive correlation|
| 0             | No similarity                    |
| -0.99         | Near perfect anti-correlation    |

Use any combination of the sample images given in `assets/` or use your own images

For debugging purposes and contributing to pre-releases, run the binary without the release flag

```bash
cargo run -- assets/sf.jpg assets/la.jpg
```
#### Contributing

Much is left to do and plan for the future of this initiative. It's meant to be a simple, easy-to-use, and performant CLI tool to compare images in a file system.

Please raise an **Issue** and make a **Pull Request** if you would like to do any of the following:

* Make this even faster
  - smarter threading strategies
  - more concurrent processes
  - refining current algorithms
  - adding additional algorithms to refine index accuracy
* Build a benchmarking feature
  - introduce unit testing natively in Rust
* Decouple components / services for better maintanability and structure
  - add Github Actions for automated CI/CD related goodies
* Build better thread management
* Have ideas for CLI subcommands or flags to expand your proposed functionality
* Update this README
* <sub>Want to do something cool and new with Rust that is beginner-friendly and applicable but still meaningful + you meet the minimum coolness factor :)</sub>

> *This repo is in **WIP** status. For more information on non-ML based image processing, read [this](https://en.wikipedia.org/wiki/Structural_similarity_index_measure)*
