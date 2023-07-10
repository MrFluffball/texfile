# texfile

A simple library to convert between klei entertainment's .tex files and png. Tested with don't starve together.

This was originally just a useless project, but it's now a crate in the obscure case someone needs to decode these.

Credit to [handsomematt's dst tools](https://github.com/handsomematt/dont-starve-tools/tree/master), he did most of the heavy lifting.

### Usage

This library exposes a single function, tex_to_png. Specify the tex file you'd like to read and the path of the resulting png file.
```rust
tex_to_png("/path/to/file.tex", "reallycoolname.png")
```