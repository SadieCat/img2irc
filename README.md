## About

`img2irc` is a tool that allows converting images to IRC formatting codes.

## Usage

```
Usage: img2irc [OPTIONS] <IMAGE> [TARGET]

Arguments:
  <IMAGE>   The path to the input image
  [TARGET]  The path to the output text

Options:
  -c, --colour-type <TYPE>   Whether to emit basic, extended, or RGB colours [default: basic]
  -e, --escape-type <TYPE>   Whether to emit raw control characters, InspIRCd config escape sequences, or InspIRCd MOTD escape sequences [default: control]
  -H, --max-height <HEIGHT>  The maximum height of the output text
  -w, --max-width <WIDTH>    The maximum width of the output text
  -t, --min-alpha <WIDTH>    The minimum alpha level to treat as a coloured tile [default: 0]
  -s, --solid                Whether to only draw solid background characters
  -h, --help                 Print help
  -V, --version              Print version
```

## License

This project is licensed under version 3 or later of the [GNU Affero General Public License](LICENSE.md).
