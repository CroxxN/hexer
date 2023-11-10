# A utility to see hexdump, but cleaner and more configurable

## How to Use?

- Basic usage

```zsh 
> hexer [options] <file_1> <file_2> <file_n>
```

- All options

```zsh
> hexer --help
```

## Currently Supported formatting

- Hex
- Octal
- Integer

## TODO:

- [x] Dynamically select column size
- [x] Option to disable formatted character display
- [x] Option to disable cannonical mode
- [x] Option to disable color formatting
- [x] Option to disable line number
- [x] Option to select formatting for line number. Eg. Hex, Oct, Integer
- [x] Option to select formatting for bytes. Eg. Hex, Oct, Integer
- [x] Bytes to picture. See: https://www.youtube.com/watch?v=4bM3Gut1hIk
- [x] Dynamically select octet size
- [x] Multiple files
- [x] Don't follow symlink
- [ ] Optimization
- [ ] man page
- [ ] hexer.conf?
