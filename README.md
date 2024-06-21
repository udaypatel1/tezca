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
<h1 align="center">Super fast image similarity indexing</h1>
</div>

### Getting Started *(beta)*

Install [rust](https://www.rust-lang.org/tools/install) if you haven't

```bash
git clone https://github.com/udaypatel1/tezca.git
cd tezca
cargo build --release
```

This will prepare the release binary on your local machine

```bash
cargo run --release assets/sf.jpg assets/la.jpg
```

| Index Value   | Meaning                          |
|---------------|----------------------------------|
| 0.99          | Near perfect positive correlation|
| 0             | No similarity                    |
| -0.99         | Near perfect anti-correlation    |

Use any combination of the sample images given in `assets/` or use your own images

For debugging and contributing to pre-releases, run the binary without the release flag

```bash
cargo run -- assets/sf.jpg assets/la.jpg
```

> *This repo is in **WIP** status. For more information on non-ML based image processing, read [this](https://en.wikipedia.org/wiki/Structural_similarity_index_measure)*
