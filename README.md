# fuzzit

Fuzzy nested git repo finder with status and diff previews


## Usage

Rich diff previews and status listed in TUI:
```sh
fuzzit
```

![20251004-0606-05 8649260](https://github.com/user-attachments/assets/993b2261-e95b-43b2-aa7f-13113e9d27b3)

-----

Simple list of one-line status summaries:
```sh
fuzzit --status
```

<img width="1365" height="922" alt="image" src="https://github.com/user-attachments/assets/a3d4b2d5-1132-40d1-9e93-f1feca4b0ae7" />

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
