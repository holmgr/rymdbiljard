# spacegolf
A simple game as part of the TFAY87 Physics course

## Documentation
To generate and open the documentation for the internal methods etc. run the
following command in the project root:

```
cargo rustdoc --open -- --no-defaults --passes collapse-docs --passes unindent-comments
```

## Running
To run the project simply run:

```
cargo run
```

## Testing
Most methods and functions in this project are unit tested using the Rust
included testing framework. To run the tests issue the following command in
the project root (or in sub folder):

```
cargo test
```

## Code style
The code styling for this project is following the Rust standard by the use
of the RustFmt project.
This provides an utility for automatically formatting the source code form the
terminal by issuing the following command:

```
cargo fmt
```
