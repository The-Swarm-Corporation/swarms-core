# swarms-core

[![CI](https://github.com/kyegomez/swarms-core/workflows/ci/badge.svg?event=push)](https://github.com/kyegomez/swarms-core/actions?query=event%3Apush+branch%3Amain+workflow%3Aci)
[![Coverage](https://codecov.io/gh/kyegomez/swarms-core/branch/main/graph/badge.svg)](https://codecov.io/gh/kyegomez/swarms-core)
[![pypi](https://img.shields.io/pypi/v/swarms-core.svg)](https://pypi.python.org/pypi/swarms-core)
[![versions](https://img.shields.io/pypi/pyversions/swarms-core.svg)](https://github.com/kyegomez/swarms-core)
[![license](https://img.shields.io/github/license/kyegomez/swarms-core.svg)](https://github.com/kyegomez/swarms-core/blob/main/LICENSE)

This package provides the core functionality for [Swarms](https://github.com/kyegomez/swarms) exeuction strategies utilizing RUST.


# Install
`pip3 install -U swarms-core`



## Getting Started

You'll need rust stable [installed](https://rustup.rs/), or rust nightly if you want to generate accurate coverage.

With rust and python 3.8+ installed, compiling swarms-core should be possible with roughly the following:

```bash
# clone this repo or your fork
git clone git@github.com:kyegomez/swarms-core.git
cd swarms-core
# create a new virtual env
python3 -m venv env
source env/bin/activate
# install dependencies and install swarms-core
make install
```

That should be it, the example shown above should now run.

You might find it useful to look at [`python/pydantic_core/_pydantic_core.pyi`](./python/pydantic_core/_pydantic_core.pyi) and
[`python/pydantic_core/core_schema.py`](./python/pydantic_core/core_schema.py) for more information on the python API,
beyond that, [`tests/`](./tests) provide a large number of examples of usage.

If you want to contribute to swarms-core, you'll want to use some other make commands:
* `make build-dev` to build the package during development
* `make build-prod` to perform an optimised build for benchmarking
* `make test` to run the tests
* `make testcov` to run the tests and generate a coverage report
* `make lint` to run the linter
* `make format` to format python and rust code
* `make` to run `format build-dev lint test`

## Test Profiling

It's possible to profile the code using the [`flamegraph` utility from `flamegraph-rs`](https://github.com/flamegraph-rs/flamegraph). (Tested on Linux.) You can install this with `cargo install flamegraph`.

Run `make build-profiling` to install a release build with debugging symbols included (needed for profiling).

Once that is built, you can profile pytest benchmarks with (e.g.):

```bash
flamegraph -- pytest tests/benchmarks/test_micro_benchmarks.py -k test_list_of_ints_core_py --benchmark-enable
```
The `flamegraph` command will produce an interactive SVG at `flamegraph.svg`.

## Releasing

1. Bump package version locally. Do not just edit `Cargo.toml` on Github, you need both `Cargo.toml` and `Cargo.lock` to be updated.
2. Make a PR for the version bump and merge it.
3. Go to https://github.com/kyegomez/swarms-core/releases and click "Draft a new release"
4. In the "Choose a tag" dropdown enter the new tag `v<the.new.version>` and select "Create new tag on publish" when the option appears.
5. Enter the release title in the form "v<the.new.version> <YYYY-MM-DD>"
6. Click Generate release notes button
7. Click Publish release
8. Go to https://github.com/kyegomez/swarms-core/actions and ensure that all build for release are done successfully.
9. Go to https://pypi.org/project/swarms-core/ and ensure that the latest release is published.
10. Done 🎉
