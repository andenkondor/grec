# grec - Git Recent Branches

A simple command-line utility that helps you navigate your Git history by showing recent branch checkouts and allowing you to easily switch between them.

## Description

`grec` parses your Git reflog to display a list of branches you've recently checked out, along with helpful metadata:

- Whether the branch has an upstream (shown as "UP")
- Whether the branch is locally accessible (shows "GONE" if not)
- The relative checkout time
- The commit message of the branch's HEAD

This makes it easy to quickly return to branches you were working on recently without having to remember their names.

## Installation

### Using Homebrew (macOS)

```bash
brew tap andenkondor/zapfhahn
brew install andenkondor/zapfhahn/grec
```

### Manual Installation

1. Download the latest release for your platform from the [Releases page](https://github.com/zapfhahn/grec/releases)
2. Extract the archive: `tar -xzf grec-<platform>-amd64.tar.gz`
3. Move the binary to a location in your PATH: `mv grec /usr/local/bin/`

### Building from Source

```bash
git clone https://github.com/zapfhahn/grec.git
cd grec
cargo build --release
```

The binary will be available at `target/release/grec`.

## Usage

Simply run `grec` in a Git repository:

```bash
grec
```

This will display a list of your recent branch checkouts and prompt you to select one to check out.

### Options

- `-c, --count <COUNT>`: Specify the number of recent branches to display (default: 10)
- `-s, --scripting`: disable all interactions, like prompting for checkout

Examples:

```bash
# Show 20 recent branches
grec -c 20

# Show recent branches without prompting for checkout
grec --scripting
```

## Features

- Displays recent Git branch checkouts
- Shows helpful metadata for each branch
- Filters out duplicates, HEAD references, and the current branch
- Allows quick checkout of any listed branch
- Colorized output for better readability

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
