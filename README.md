# Unofficial Typst Package Manager

> Alias "utpm"

Docs in WIP!

# What is that?

UTPM is a package manager for [local](https://github.com/typst/packages#local-packages) and remote (wip) packages.
The goal is to manage packages to use the new feature of typst, [importing other packages from different folders](https://typst.app/docs/reference/scripting/#packages)

# Why I didn't work on it for the last months

I was missing a lot of times and a lot of motivation. Now I got time and motivation so you will see some updates!

# How to use it?

You need two things, first create an `typst.toml` file by using `utpm create`:

```bash
$ utpm create
```

Example:

https://github.com/ThumusLive/utpm/assets/42680097/473f4826-773b-4b2c-9a31-3af5756799c2

Modify this file to match your project and then:

And finally, you need to copy the directory to the "special" directory by using `utpm link`: 

```bash
$ utpm link
```
Example:

https://github.com/ThumusLive/utpm/assets/42680097/92c06cba-928f-4ffb-b2ca-dae67ff7b32d

# Install

You will need Cargo and Rust.

Simpliest way :

```bash
git clone https://github.com/ThumusLive/utpm.git &&
cd utpm &&
cargo install --path .
```

There is a `build.sh` to install/update the project.

# TODO:

- [x] Reimpl errors
- [x] Last typst version
- [ ] More commands:
  - [ ] Unlink
  - [x] List
- [x] Create `typst.toml` by asking questions
- [ ] ""pre-export"" package by giving them what they need
- [x] use semver
- [ ] Use custom packages namespace (e.g "@custom/example:1.0.1")
- [ ] Fix typo

# Contribution

If you want to help me dev this package, simply make an issue or a PR

By using this app, you contribute to it, thank you! <3
