# Unofficial Typst Package Manager

> Alias "utpm"

# What is that?

UTPM is a package manager for [local](https://github.com/typst/packages#local-packages) and remote packages.
The goal is to manage packages to use the new feature of typst, [importing other packages from different folders](https://typst.app/docs/reference/scripting/#packages)

# How to use it?

You need two things, first create an `typst.toml` file by using `utpm create`:

```bash
$ utpm create
```

Modify this file to match your project and finally, you need to copy the directory to the "special" directory by using `utpm link`: 

```bash
$ utpm link
```

# Install

You will need Cargo and Rust.

Simpliest way :

```bash
git clone https://github.com/Thumuss/utpm.git &&
cd utpm &&
cargo install --path .
```

There is a `build.sh` to install/update the project.

# TODO:

See TODO.md

# Contribution

If you want to help me dev this package, simply make an issue or a PR

By using this app, you contribute to it, thank you! <3
