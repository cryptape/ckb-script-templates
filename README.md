# ckb-script-templates

This repository keeps a series of CKB script templates that can be inflated via [cargo-generate](https://github.com/cargo-generate/cargo-generate). Those templates enable a native development flow on mainstream Linux, macOS and Windows machines using stock version of latest stable Rust & clang C compiler.

## Usage

### Dependencies

The following dependencies are required for the templates:

* `git`, `make`, `sed`, `bash`, `sha256sum` and others Unix utilities. Refer to the documentation for your operating systems for how to install them. Chances are your system might already have them.
* `Rust`: latest stable Rust installed via [rustup](https://rustup.rs/) should work. Make sure you have `riscv64` target installed via: `rustup target add riscv64imac-unknown-none-elf`
* `Clang`: make sure you have clang 16+ installed, sample installtion steps for selected platforms are:
    + Debian / Ubuntu: `wget https://apt.llvm.org/llvm.sh && chmod +x llvm.sh && sudo ./llvm.sh 16 && rm llvm.sh`
    + Fedora 39+: `sudo dnf -y install clang`
    + Archlinux: `sudo pacman --noconfirm -Syu clang`
    + macOS: `brew install llvm@16`
    + Windows(with [Scoop](scoop install llvm yasm)): `scoop install llvm yasm`
* `cargo-generate`: You can install this via `cargo install cargo-generate`, or follow the steps [here](https://cargo-generate.github.io/cargo-generate/installation.html)

### Creating workspace

To generate a workspace template, use the following command:

```
$ cargo generate gh:xxuejie/ckb-script-templates workspace
⚠️   Favorite `gh:xxuejie/ckb-script-templates` not found in config, using it as a git repository: https://github.com/xxuejie/ckb-script-templates.git
🤷   Project Name: my-first-contract-workspace
🔧   Destination: /tmp/my-first-contract-workspace ...
🔧   project-name: my-first-contract-workspace ...
🔧   Generating template ...
🔧   Moving generated files into: `/tmp/my-first-contract-workspace`...
🔧   Initializing a fresh Git repository
✨   Done! New project created /tmp/my-first-contract-workspace
```

Or you can manually specify the name and skip the prompt:

```
$ cargo generate gh:xxuejie/ckb-script-templates workspace --name my-first-contract-workspace
⚠️   Favorite `gh:xxuejie/ckb-script-templates` not found in config, using it as a git repository: https://github.com/xxuejie/ckb-script-templates.git
🔧   Destination: /tmp/my-first-contract-workspace ...
🔧   project-name: my-first-contract-workspace ...
🔧   Generating template ...
🔧   Moving generated files into: `/tmp/my-first-contract-workspace`...
🔧   Initializing a fresh Git repository
✨   Done! New project created /tmp/my-first-contract-workspace
```

This is probably the only longer command you will deal with when using the templates repository. You can save them as an alias in your shell:

```
$ alias create-ckb-scripts="cargo generate gh:xxuejie/ckb-script-templates workspace"
$ create-ckb-scripts
⚠️   Favorite `gh:xxuejie/ckb-script-templates` not found in config, using it as a git repository: https://github.com/xxuejie/ckb-script-templates.git
🤷   Project Name: my-first-contract-workspace
🔧   Destination: /tmp/my-first-contract-workspace ...
🔧   project-name: my-first-contract-workspace ...
🔧   Generating template ...
🔧   Moving generated files into: `/tmp/my-first-contract-workspace`...
🔧   Initializing a fresh Git repository
✨   Done! New project created /tmp/my-first-contract-workspace
```

### Working in the workspace

First, navigate to the workspace directory you just created:

```
$ cd my-first-contract-workspace
```

First thing you can notice, is that we have created a standard Rust workspace project:

```
$ tree .
.
├── Cargo.toml
├── Makefile
├── scripts
│   └── find_clang
└── tests
    ├── Cargo.toml
    └── src
        ├── lib.rs
        └── tests.rs

4 directories, 6 files
```

The only exception here, is that a `Makefile`, together with a `scripts` folder has been created to simplify contract building.

We can use `make generate` to create our first contract:

```
$ make generate
🤷   Project Name: first-contract
🔧   Destination: /tmp/my-first-contract-workspace/contracts/first-contract ...
🔧   project-name: first-contract ...
🔧   Generating template ...
🔧   Moving generated files into: `/tmp/my-first-contract-workspace/contracts/first-contract`...
🔧   Initializing a fresh Git repository
✨   Done! New project created /tmp/my-first-contract-workspace/contracts/first-contract
Please update workspace-level Cargo.toml so members include the newly created crate!
```

As the log line hints, after creating the contract, we will need to manually update `Cargo.toml` file to contain the newly created contract crate. When you finished editing, `Cargo.toml` file should look like following:

```
$ cat Cargo.toml
[workspace]
resolver = "2"

members = [
  # Please don't remove the following line, we use it to automatically
  # detect insertion point for newly generated crates.
  # @@INSERTION_POINT@@
  "contracts/first-contract",
  "tests",
]

[profile.release]
overflow-checks = true
strip = true
codegen-units = 1
```

You can also supply the contract create name when executing the make task:

```
$ make generate CRATE=second-contract
🔧   Destination: /tmp/my-first-contract-workspace/contracts/second-contract ...
🔧   project-name: second-contract ...
🔧   Generating template ...
🔧   Moving generated files into: `/tmp/my-first-contract-workspace/contracts/second-contract`...
🔧   Initializing a fresh Git repository
✨   Done! New project created /tmp/my-first-contract-workspace/contracts/second-contract
```

Note that when you supply the contract crate name, our `Makefile` will be smart enough to automatically insert the crate in to `Cargo.toml` file, so you don't have to edit it manually.

Now you can build the contracts(or adjust parameters):

```
$ make build
$ make build MODE=debug                   # for debug build
$ make build CUSTOM_RUSTFLAGS=""          # release build without debug assertions
$ make build CARGO_ARGS="--verbose"       # release build with `--verbose` attached to cargo command, you can use other arguments accepted by cargo
$ make build CONTRACT=second-contract     # build a single contract
$ make build CLEAN_BUILD_DIR_FIRST=false  # keep old untouched binaries
$ make build CLANG=clang-17               # use a specific clang version to build C code
```

We have prepared a `tests` crate where you can also add contract tests. If you want to run the tests, run the following command:

```
make test
```

The templates provided here, use the same conventions as `ckb-native-build-sample` project, so feel free to refer to the more detailed [usage](https://github.com/xxuejie/ckb-native-build-sample?tab=readme-ov-file#usage) doc in the sample project.
