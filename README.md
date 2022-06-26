## About

`img2irc` is a tool that allows converting images to IRC formatting codes.

## Usage

```
USAGE:
    img2irc [OPTIONS] <IMAGE> [TARGET]

ARGS:
    <IMAGE>     The path to the input image
    <TARGET>    The path to the output text

OPTIONS:
    -c, --colour-type <TYPE>     Whether to emit basic, extended, or RGB colours [default: basic]
    -e, --escape-type <TYPE>     Whether to emit raw control characters, InspIRCd config escape
                                 sequences, or InspIRCd MOTD escape sequences [default: control]
    -h, --max-height <HEIGHT>    The maximum height of the output text
        --help                   Print help information
    -t, --min-alpha <WIDTH>      The minimum alpha level to treat as a coloured tile [default: 0]
    -V, --version                Print version information
    -w, --max-width <WIDTH>      The maximum width of the output text
```

## License

This project is licensed under version 3 or later of the [GNU Affero General Public License](LICENSE.md).
