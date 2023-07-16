# Unofficial Typst Package Manager
> Alias "utpm"

Docs in WIP!

# What is that?

UTPM is a package manager for [local](https://github.com/typst/packages#local-packages) and remote (wip) packages.
The goal is to manage packages to use the new feature of typst, [importing other packages from different folders](https://typst.app/docs/reference/scripting/#packages)

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

Download the project and then: 

```bash
$ cargo build -r
```

You'll find the file in `target/release/unofficial-typst-package-manager`.

On Linux, you can move the file to your PATH by moving the file for example:

```bash
$ sudo mv ./target/release/unofficial-typst-package-manager /usr/bin/utpm
```