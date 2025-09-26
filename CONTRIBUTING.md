# Contributing

- Tabs not spaces
- Use the rustfmt.toml file that is in the root dir of the repo
- Keep code in related folders
  - I.E. TUI related stuff goes in the `src/tui/` folder, networking related stuff goes in the `src/net/` folder
- Make sure that any local editor config files/directories (like `.vscode/`) are ignored by Git
- Issues for bugs, Discussions for features
- Limit the dependency chain as much as possible (so don't add 50 deps in a single PR, theoretically, we only really should have a couple crates as dependencies for stuff like concurrency and networking)
- Absolutely no AI
- Document your code with comments or entries in the `docs/` folder
  - doc files are either `.md`, single doc file for each src file, `doc/` directory structure should mirror `src/` (so `src/tui/tui.rs` will have a doc file in `docs/tui/` called `tui.md`)
- Remove ecessive trailing newlines or whitespace (so like reduce those 5 empty lines you had separating two sections of code to a single line)

# TODO:
- [ ] RFC 1459
  - [ ] IRC Client Protocol
  - [ ] IRC Server Protocol
- [ ] GUI (OpenGL or Vulkan?)
- [ ] TUI (Ratatui)
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
