# fxg

## todo :)

- publish to cargo
- add progress indicator
- publish vscode extension
- write the wiki

## Your first FXG blog

You will need

- Cargo
- A text editor ([VSCode](https://code.visualstudio.com/) is recommended, but you can use anything)
- 2 tbsp salt

Steps:

1. Open a terminal
2. In this terminal, run the command `cargo install fxg`
3. When this finishes, run `fxg new my_first_fxg_blog`
4. Open VSCode in the newly created folder.
5. All features are used in this one file, you can view this as a [cheatsheet](https://gist.github.com/zTags/ba3f4ef67a1593f1b71fa33edcebaa2e)
6. run `fxg build --start` to run a webserver with your blog

## CLI Reference

### Build

`fxg build <directory?> --start?`: builds a project, if no directory was provided, uses your current working directory.
If `--start` is provided: Starts a server which hosts your blog.

### New

`fxg new <name>`: creates a new project in a directory with the same name

## Installing the VSCode extensions

todo
