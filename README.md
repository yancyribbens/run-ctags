# run-ctags (Rust UNiversal ctags)

Cargo is used by `run-ctags` to list all files on which your Rust project depends.  Once all dependent paths are discovered, the results are piped to [universal-ctags] to build a tags file.

This crate is intended for use with Vim or Emacs in combination with [universal-ctags].

## Installation
Please follow the installation instructions for [universal-ctags].

[universal-ctags]: https://github.com/universal-ctags/ctags

After [universal-ctags] has been installed, install `run-ctags`.

### Install run-ctags
```
cargo install run-ctags
```

## Usage
Once [universal-ctags] is installed, use the following command to enumerate all root directories and then pipe the results to ctags.  From there, ctags will recursively search through all files and folders, starting from the root directory, and create a file named ctags in your project's folder.

From the root directory of your Rust project, run the following command:

### build the tags file
```sh
run-ctags | ctags -R -L -
```

This command pipes all of your project dependencies to ctags which then in turn builds an index for your project saved in a file named `tags`.  If you now open Vim in your project's root directory, you can use ctags to jump in and out of different definitions.

Below are a few helpful commands to run from within Vim after the tags file has been created.  Place the cursor over a piece of code and press `Ctrl+]` to jump to the definition.

### commands
```
Ctrl+] - Jump to the definition.
Ctrl+T - Jump back from the definition.
Ctrl+W Ctrl+] - Open the definition in a horizontal split.
g+] - List all possible definitions.
```

Note, Vim can only find the tags file if you start Vim in the project root.  However, the following command can be added to `.vimrc` which allows Vim to open the tags file from any subdirectory:

### .vimrc
```
set tags=./tags;/
```
