# Contributing

- Tabs not spaces
- Use the rustfmt.toml file that is in the root dir of the repo
- Keep code in related folders
  - I.E. TUI related stuff goes in the `src/tui/` folder, networking related stuff goes in the `src/net/` folder
- Make sure that any local editor config files/directories (like `.vscode/`) are ignored by Git
- Issues for bugs, Discussions for features
- Limit the dependency chain as much as possible (so don't add 50 dependencies in a single PR, theoretically, we only really should have a couple crates as dependencies for stuff like concurrency, GUI/TUI, and networking)
- Absolutely no AI
- Document your code with comments and an entry/entries in the `docs/` folder
  - doc files are either `.md`, single doc file for each src file, `doc/` directory structure should mirror `src/` (so `src/tui/tui.rs` will have a doc file in `docs/tui/` called `tui.md`)
- Remove excessive trailing newlines or whitespace (so like reduce those 5 empty lines you had separating two sections of code to a single line)
- Unix-style line endings (`LF`), NO `CRLF`, I will blow you up if I see `^M` when I review your PR (/j)
- In order to keep Pull Requests easy to manage, `main.rs` should be the only shared `dev` branch file that should ever have any changes to it
  - Topic branch (like `network` or `tui`) forks (Ex: `network-nadelio`), can have multiple conflicting files, this is unavoidable due to the possibility of people working in the same section of code at the same time
- Topic branches will be created as needed by the first person to work on that topic, a topic is any TODO element that is level 1 or less (level as in level of indents below), TODO elements with <2 indents are considered *tasks* and are handled in topic branch forks by the fork maintainer
  - Topic branches need to be fully current with the `dev` branch and have no actively maintained forks before being merged to `dev`

# TODO:
- [ ] RFC 1459
  - [ ] IRC Client Protocol
  - [ ] IRC Server Protocol
- [ ] GUI (OpenGL or Vulkan?)
- [ ] TUI (Ratatui)
  - [ ] Add function for quickly sending CLIENT/SERVER info messages for debugging or otherwise to the message output buffer
  - [X] Client side syntax highlighting
  - [ ] Message box frame
  - [ ] Channel box frame
  - [ ] User input frame
  - [ ] Mode frame
  - [ ] NAV mode
  - [ ] MSG mode
  - [ ] Member box frame
  - [ ] Current server box frame
  - [ ] Message timestamps
  - [ ] Message usernames

### Mockup of TUI in MSG mode
<img width="511" height="316" alt="image" src="https://github.com/user-attachments/assets/a415a78d-5b7d-4f43-8961-692b2045706e" />

### Mockup of TUI in NAV mode
<img width="512" height="322" alt="image" src="https://github.com/user-attachments/assets/68b2cecb-7ede-46bd-be62-41857b35db90" />
