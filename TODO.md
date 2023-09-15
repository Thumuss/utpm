# TODO list

## Done: 

- [x] Reimpl errors
- [x] Last typst version
- [x] More commands:
  - [x] Unlink
  - [x] List
- [x] Create `typst.toml` by asking questions
- [x] use semver
- [x] Use custom packages namespace (e.g "@custom/example:1.0.1")
- [x] Fix typo

## TODO:

- [ ] ""pre-export"" package by giving them what they need 
- [ ] Documentation for developpers
  - [ ] utils.rs
  - [ ] main.rs
  - [ ] commands.rs → remake it?
    - [ ] commands/create.rs
    - [ ] commands/link.rs
    - [ ] commands/list.rs
    - [ ] commands/unlink.rs
    - [ ] commands/install.rs (wip)
- [ ] Download packages from unofficial repos, see #3
  - [ ] git2-rs
  - [ ] Pre-fetch to see `typst.toml`
  - [ ] Dependencies managed
  - [ ] use utpm namespace to use libs (or portable so without any links)
  - [ ] Maybe a launchable version from utpm to link packages?
- [ ] Portable version and only installable version
  - [ ] Integrate install
  - [ ] And all of todos from above with #3