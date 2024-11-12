Cargo
Enjoy this cheat sheet at its fullest within Dash, the macOS documentation browser.
Handy cheat sheet for Rust package manager Cargo (cargo)

Based on Cargo version:

$ cargo --version
cargo 1.29.0 (524a578d7 2018-08-05)
Usage
--version

Shows version information, hash and date, see also the introduction above

--list

Lists all installed and available commands

--verbose

Enable verbose output. Shorthand -v and -vv for more (very) verbose output

--quiet

Disable output on STDOUT

--color WHEN

Enable colours, auto, always or never

--help

Shows help message

--locked

Require Cargo.lock is up to date

--frozen

Require Cargo.lock and cache are up to date is up to date

-Z FLAG

Unstable (nightly-only) flags to Cargo, see "cargo -Z help" for details for further information on unstable flags

--explain CODE

Executes rustc --explain CODE

Start a Rust Project
new --bin APPLICATIONNAME

Create new application/executable based project

new --lib LIBRARYNAME

Create new library based project, this is actually the default and --lib can be omitted

init --bin

Initialize a new application/executable in current directory

init --lib

Initialize a library in current directory

Build a Rust Project
build

Build a Rust project

build -j JOBS

Build a Rust project in parallel with mutiple jobs

run

Executes benchmark of project, requires tests

bench

Executes benchmark of project, requires tests

check

Analyze the current project and report possible errors, but does not build object files

test

Executes project tests, requires tests

doc

Builds projects documentation, requires inlines documentation in source files

clean

Clean out the build, by removing the target/ directory

Maintain a Rust Project
search

Search for crates

install

Install a Rust binary

install CREATENAME

Install a named crate, see search above

fetch --list

Lists installed crates

install --list

Lists installed crates

Maintain a Rust Crate
package

Package this project into a distributable tarball

publish

Upload a package to the registry

yank

Remove a pushed crate from the index

Environment Variables
CARGO_HOME

Cargo maintains a local cache of the registry index and of git checkouts of crates. By default these are stored under $HOME/.cargo, this variable overrides the location of this directory. Once a crate is cached it will not removed by the clean command

CARGO_TARGET_DIR

Location of where to place generated artifacts, relative to the current working directory

RUSTC

Instead of running rustc, Cargo will execute specified compiler instead

RUSTC_WRAPPER

Instead of running rustc, Cargo will execute specified wrapper instead, passing as its commandline arguments the rustc invocation, with the first argument being rustc

RUSTDOC

Instead of running rustdoc, Cargo will execute specified rustdoc instance

RUSTDOCFLAGS

A space-separated list of custom flags to pass to rustdoc invocations that Cargo performs. In contrast to cargo rustdoc, this is useful for passing a flag to all rustdoc instances

RUSTFLAGS

A space-separated list of custom flags to pass to compiler invocations that Cargo performs. In contrast to cargo rustc, this is useful for passing a flag to all compiler instances

CARGO_INCREMENTAL

If set to 1 then Cargo will force incremental compilation to be enabled for the current compilation, and when set to 0 it will force disabling it. If this env var is not present then Cargo defaults will be used

CARGO_CACHE_RUSTC_INFO

If set to 0 then Cargo will not attempt to cache compiler version information

Notes
For more information on Cargo see the official documentation
For more information on the Cargo environment variables see the official documentation
For access to the Cargo source code, visit the repository on GitHub
You can modify and improve this cheat sheet here