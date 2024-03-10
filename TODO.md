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

## V2.1:

- [X] ""pre-export"" package by giving them what they need 
- [ ] Documentation for developpers
  - [ ] utils.rs
  - [ ] main.rs
  - [ ] commands.rs → remake it?
    - [ ] commands/create.rs
    - [ ] commands/link.rs
    - [ ] commands/list.rs
    - [ ] commands/unlink.rs
    - [ ] commands/install.rs
- [x] Download packages from unofficial repos, see #3
  - [x] git2-rs
  - [x] Dependencies managed
  - [x] use utpm namespace to use libs (or portable so without any links) → It wouldn't be as good as it sounds, typst can't use package outside the data folder
  - [x] Maybe a launchable version from utpm to link packages?
- [x] Portable version and only installable version
  - [x] Integrate install
  - [x] And all of todos from above with #3
- [ ] New commands for install:
  - [ ] Info.rs
  - [ ] Update, (using semver)
  - [x] Bulk delete
  - [ ] Clean?
- [ ] Maybe a listing dependencies system? -> Track every dependencies to delete when they aren't used -> Not for now
- [ ] Templates (first impl) -> Not now → V3
- [ ] JSON only mode 

## V3

This update will introduce documentations, a better handling error system, JSON and some commands. 

- [x] Better handling errors (json, string, toml maybe)
- [ ] Maybe a listing dependencies system? -> Track every dependencies to delete when they aren't used
- [ ] Create a global and local configuration instead of using typst.toml file. It can become harder to 
- [x] JSON only mode 
- [x] Remake clap
- [ ] Documentation for developpers and users
  - [ ] utils.rs
  - [ ] main.rs
  - [ ] commands.rs → remake it?
    - [ ] commands/create.rs
    - [ ] commands/link.rs
    - [ ] commands/list.rs
    - [ ] commands/unlink.rs
    - [ ] commands/install.rs
- [ ] New commands for install:
  - [ ] Info.rs -> Partial impl for now
  - [ ] Update, (using semver) → \w listing dependencies
  - [ ] Clean?


## V4 (2024.03.10)

As this date, a new version of typst as been release (v0.11.0-rc1 (fe94bd85)) with a new system of template.
For now on, this version of `utpm` will focuse on adapting the new system and be compatible with the previous system.

If time isn't running out, I'll add quality of life improvement such as a `Context` system, new commands to go along with the `typst init` command and Dockerise everything (kubernetes included).

The main focus will be : 
- [ ] Add templates in `utpm`
- [ ] Compatibility with older version of typst
- [ ] Fix current bugs

If we got time, I'll add theses things:
- [ ] Docker, Compose and Kubernetes files (and examples)
- [ ] `Context`, it will change the actual structure of handling json and errors.
- [ ] ENV compatible.
- [ ] get along with `typst init`

If you have any ideas to improve `utpm`, i'll take it <3