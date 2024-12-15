# Terminal Maze

This is a terminal maze game I wrote in Rust for fun.

### Features

- Randomly generated maze
- Customize maze height and width
- Tracks your path through the maze
- Can help you solve the maze
- Watch the game go through the maze

### Build

```shell
cargo build
```

### Run

```shell
cargo run
```

### Help

```shell
cargo run -- -h
```

```text
Terminal maze game

Usage: maze [OPTIONS]

Options:
      --height <HEIGHT>  [default: 10]
      --width <WIDTH>    [default: 10]
  -b, --breadcrumbs      Show red dots along your path
  -m, --mode <MODE>      Watch the game solve the maze [default: play] [possible values: play, watch]
  -h, --help             Print help
```

### Controls

- Use <kbd>&larr;</kbd> <kbd>&uarr;</kbd> <kbd>&darr;</kbd> <kbd>&rarr;</kbd> to move.
- Use <kbd>Esc</kbd> or <kbd>q</kbd> to quit.
- Use <kbd>Space</kbd> to get a hint!
- Use <kbd>b</kbd> to show a trail of breadcrumbs.