# brainrust

## Usage

`cargo r -- <file> <mode>`

Where `mode` is either `run`, for interpretation, or `comp` for compilation.

Compilation will produce an `out.s` file, which can be assembled and linked into a elf64 binary be running `src/build.sh` on it.
