# Quick Note
## `qn`
### Install

This is currently for my personal use. I may push breaking changes at any time.

If you want to use it, bring down the code and run the following from the project's root directory:

`cargo install --path .`

That will install it in your `~/.cargo/bin/`

### Usage

Pass a `--category` and your `--note`.

You must have a `$HOME/notes/` directory and subdirectories with names matching the categories you want.

For each category, a file named `quick_notes.txt` will be created when you first use the category. After that, each note in that category will be appended to that file.

````console
tech/Rusty> qn -c "rust" -n "Remember to rewrite it in Rust"
````

````console
baseball/dodgers> qn --help
q-note 0.1.0
Joel Palmer <rust_nvim@pm.me>
qn (quick note) is a simple, fast and user-friendly way to save notes

USAGE:
    qn --note <NOTE> --category <CATEGORY>

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
- [ ] Integrate with **Todoist** API*
- [ ] Integrate with **Google Calendar** API*

_* I might just script this functionality locally. But, you never know._
