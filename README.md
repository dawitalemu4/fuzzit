# fuzzit

Fuzzy nested git repo finder with status and diff previews


## Usage

Rich diff previews and status list:
```sh
fuzzit
```

<!---
gif
-->

Simple list of one-line status summaries:
```sh
fuzzit status
```

<!---
gif
-->

Filter output by specifying starting path:
```sh
# Starting path will be requested and saved on first use
# Relative paths (ex: PATH="./big-folder-with-projects" from ~/dev) will work
PATH="~/dev/big-folder-with-projects" fuzzit
```


## Installation

```sh
brew install fuzzit
```

or

```sh
curl -L https://github.com/dawitalemu4/fuzzit/releases/download/v1.0.0/fuzzit_1.0.0_windows_x86_64.zip
```


## License

This project is licensed under the Creative Commons Attribution-NonCommercial 4.0 International Public License - see the [LICENSE.txt](https://github.com/dawitalemu4/fuzzit/blob/main/LICENSE.txt).
