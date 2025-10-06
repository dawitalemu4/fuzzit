# fuzzit

Fuzzy nested git repo finder with status and diff previews


## Usage

Rich diff previews and status listed in TUI:
```sh
fuzzit
```

![fuzzit](https://raw.githubusercontent.com/dawitalemu4/fuzzit/main/assets/tui.gif)

-----

Simple list of one-line status summaries:
```sh
fuzzit --status
```

![fuzzit --status](https://raw.githubusercontent.com/dawitalemu4/fuzzit/main/assets/status.png)

-----

Filter output by specifying starting path:
```sh
# Relative paths (ex: FUZZIT_PATH="./folder-with-many-projects" from ~/dev) will work
FUZZIT_PATH="~/dev/folder-with-many-projects" fuzzit
```


## Installation

```sh
cargo install fuzzit
```

Add your dev folder as `FUZZIT_BASE_PATH` to your environment (ex: `export FUZZIT_BASE_PATH="~/dev"` in ~/.zshrc)


## License

This project is licensed under the Creative Commons Attribution-NonCommercial 4.0 International Public License - see the [LICENSE.txt](https://github.com/dawitalemu4/fuzzit/blob/main/LICENSE.txt).
