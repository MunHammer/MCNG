# Rules
## Don't be a dick
  - Self explantory
## Be respectful
  - Just in case you didn't see the above
## To contribute:
  - Main must **always** be stable.
  1. Make a fork/branch (branch if you're a contributor)
  2. Add your name to the utility that you're contributing to.
  3. Contribute, add code, edit it, whatever
  4. Lint
      - The Cargo.toml for rust projects must lint:
          - lints.clippy.cargo
          - lints.clippy.pedantic
          - lints.rust.missing_docs
      - You may stop some errors, but you may not be too excessive in your ignorance
      - If it's Rust, `cargo clippy`.
      - If it's C, `gcc -Wall`.
      - If there are any errors, go back to step 3.
  5. Add the documentation
      - If it's Rust, `cargo doc`
      - If it's C, use doxygen (I haven't used doxygen)
  4. Make a pull request to the main repo/branch
  5. Wait
     1. A(nother) contributor will review your pull request
     2. If it's merged, it's merged
     3. If it isn't, go back to step 3
  6. You're done!

# Documentation
- The tutorials & end-user documentation must be made with [mdbook](https://github.com/rust-lang/mdBook)
- Dev documentation for Rust is made with `cargo doc`
- Dev documentation for C is made with doxygen
- Other dev documentation is made with plain markdown

# This project will use the following programming languages:
## Core langs
- Rust
- C
## Documentation (output)
- Typescript
- HTML
- CSS
- Markdown
## Other
- Makefile (for compiling the C code)
- TOML (End user config files)

## The file structure will be as so:
```
.
|-- CONTRIBUTION.md
|-- COPYING
|-- README.md
|-- util-claimer.md
|-- docs
|   |-- index.html
|-- SingleProject
|   |-- src
|   |   |-- main.c
|   |-- tests
|   |   |-- test_0.c
|   |-- docs
|   |   |-- index.html
|   |-- README.md
|   |-- TODO.md
|   |-- COPYING
|   |-- makefile # If the tool is made in C or C++
|-- Another project, structured similarly
```
