# TODO list

## V2: 

- [x] Reimpl errors
- [x] Last typst version
- [x] More commands:
  - [x] Unlink
  - [x] List
- [x] Create `typst.toml` by asking questions
- [x] use semver
- [x] Use custom packages namespace (e.g "@custom/example:1.0.1")
- [x] Fix typo

## V3:

- [ ] ""pre-export"" package by giving them what they need 
- [ ] Documentation for developpers
  - [ ] utils.rs
  - [ ] main.rs
  - [ ] commands.rs → remake it?
    - [ ] commands/create.rs
    - [ ] commands/link.rs
    - [ ] commands/list.rs
    - [ ] commands/unlink.rs
    - [ ] commands/install.rs
- [ ] Download packages from unofficial repos, see #3
  - [x] git2-rs -> Not the best use of it
  - [x] Dependencies managed
  - [x] use utpm namespace to use libs (or portable so without any links) → It wouldn't be as good as it sounds, typst can't use package outside the data folder
  - [x] Maybe a launchable version from utpm to link packages?
- [x] Portable version and only installable version
  - [x] Integrate install
  - [x] And all of todos from above with #3
- [ ] New commands for install:
  - [ ] Info.rs
  - [ ] Update, (using semver)
  - [ ] Bulk delete ()
  - [ ] Clean?
- [ ] Maybe a listing dependencies system? -> Track every dependencies to delete when they aren't used
