# Quick Note
<img align="right" width="100" height="100" src="https://user-images.githubusercontent.com/8049061/155224899-71324823-4cc0-431a-90e5-63e3c51af05f.png">
## `qn`
### Install

This is currently for my personal use. I may push breaking changes at any time.

If you want to use it, bring down the code and run the following from the project's root directory:

`cargo install --path .`

That will install it in your `~/.cargo/bin/`

### Usage

Pass a `--category` and your `--note`.

You must have a `$HOME/notes/` directory (or set your own `<ROOT>`) and you need subdirectories with names matching the categories you want.

For each category, a file named `quick_notes.txt` will be created when you first use the category. After that, each note in that category will be appended to that file.

````console
tech/Rusty> qn -c "rust" -n "Remember to rewrite it in Rust"
````

````console
baseball/dodgers> qn --help
q-note 0.1.0
Joel Palmer <rust_nvim@pm.me>
qn (quick note) is a simple, fast and user-friendly way to save organized notes

USAGE:
    qn --note <NOTE> --category <CATEGORY> [ROOT]

ARGS:
    <ROOT>    Root directory of notes: `$HOME/<ROOT>` [default: notes]

OPTIONS:
    -c, --category <CATEGORY>    Category for note to be saved under
    -h, --help                   Print help information
    -n, --note <NOTE>            Text of note
    -V, --version                Print version information
````

### TODOS ✓
- [x] Basic functionality
- [x] Take a category that maps to a subdirectory
- [ ] Allow for category creation and/or validation
- [ ] Allow for configuration of locations and categories
	- [x] Allow users to set their root directory
- [ ] Neovim plugin
  - [ ] Telescope extension?
- [ ] Minor CRUD
- [ ] Integrate with **Todoist** API*
- [ ] Integrate with **Google Calendar** API*

_* I might just script this functionality locally. But, you never know._

