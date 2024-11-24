# rquickjs module

[![github](https://img.shields.io/badge/github-delskayn/rquickjs-8da0cb.svg?style=for-the-badge&logo=github)](https://github.com/rquickjs/rquickjs-module)
[![crates](https://img.shields.io/crates/v/rquickjs.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/rquickjs-module)

This is an extension to [rquickjs](https://github.com/DelSkayn/rquickjs) to allow the ecosystem to create more unified Rust modules.

The goal was to create a better version of [`ModuleDef`](https://docs.rs/rquickjs/latest/rquickjs/module/trait.ModuleDef.html) that would allow it to have options as input and set global.

For example, a `fetch` module using `ModuleDefExt` could set a global `fetch` function and have as options an allowlist of domains.

## Usage
