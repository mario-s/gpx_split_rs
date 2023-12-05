## GPX file splitter

This tool can split a route or track of GPX file into smaller chunks. It will create new files, where each one has a suffix to the filename like so _FILENAME_1.gpx_.

Inspired by my [earlier project](https://github.com/mario-s/gpx_split_py) which uses Python, but this time in Rust.

### Help
`gpx_split --help`

### Verbose
For more information during the process, run the application with the environment variable `RUST_LOG`

`RUST_LOG=debug gpx_split <ARGS>`