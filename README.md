# Unofficial Typst Package Manager

> Alias "utpm"

Docs in WIP!

# What is that?

UTPM is a package manager for [local](https://github.com/typst/packages#local-packages) and remote (wip) packages.
The goal is to manage packages to use the new feature of typst, [importing other packages from different folders](https://typst.app/docs/reference/scripting/#packages)

# Why I didn't work on it for the last months

I was missing a lot of times and a lot of motivation. Now I got time and motivation so you will see some updates!

# How to use it?

Simple as:

> Create a typst.toml file

```bash
$ utpm create
```

Modify this file to match your project and then:

> Move it

```bash
$ utpm link
```

# Install

You will need Cargo and Rust.

Simpliest way :

```bash
git clone https://github.com/ThumusLive/utpm.git &&
cd utpm &&
cargo install --path .
```

# TODO:

- [x] Reimpl errors
- [x] Last typst version
- [ ] More commands:
  - [ ] unlink
  - [x] List
- [x] Create `typst.toml` by asking questions
- [ ] ""pre-export"" package by giving them what they need
- [x] use semver

# Contribution

If you want to help me dev this package, simply make an issue or a PR

By using this app, you contribute to it, thank you! <3
