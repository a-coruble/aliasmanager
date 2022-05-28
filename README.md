# AliasManager
![Language](https://img.shields.io/badge/Written_in_Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Status](https://img.shields.io/badge/status-WIP-orange?style=for-the-badge)
[![GitHub issues](https://img.shields.io/github/issues/a-coruble/aliasmanager?style=for-the-badge)](https://github.com/a-coruble/aliasmanager/issues)

AliasManager is a small CLI utility to quickly add new shell aliases without
the burden of manually editing your shell configuration file.

## Usage

```
AliasManager
    A tiny utilitary CLI tool to quickly create new aliases in your shell config file.
    Please note that this programs uses the `SHELL_CONFIGURATION_FILE_PATH` environment
    variable to determine where to write the alias.

USAGE:
    aliasmanager [--OPTIONS] <trigger> <command>
                
OPTIONS:
    --help: prints this help message

EXAMPLE:
    aliasmanager rmf "rm -rf"
```

## Installation

### From source

1. Make sure you have Rust v1.61.0 or higher
2. Clone the repository
3. From inside the repository, execute the following command: `cargo install --path .`

### Without cloning

1. Make sure you have Rust v1.61.0 or higher
2. Execute the following command: `cargo install --git https://github.com/a-coruble/aliasmanager`

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.