# editenv

_get/set os env value in rust._

# Contents

1. [Usage](#usage)
   - [Examples](#examples)
2. [Dependencies](#Dependencies)
3. [License](#license)

# Usage

Build editenv with `cargo build --release` and move the executable from
`target/release/editenv(.exe)` to a directory with environment access. After this
you can use editenv from the command line:

```shell
editenv set key value
editenv get key
```

## Examples

set "OLLAMA_MODELS" to "G:\data\LLM":

```shell
editenv set OLLAMA_MODELS "G:\data\LLM"
```

to get "OLLAMA_MODELS" 's value

```shell
editenv get OLLAMA_MODELS
```

# Dependencies

editenv uses the following libraries:

- [clap](https://crates.io/crates/clap) - Command line argument parsing.
- [winreg](https://crates.io/crates/winreg) - Rust bindings to MS Windows Registry API.

# Build-Dependencies

- [winres](https://crates.io/crates/winres) - Create and set windows icons and metadata for executables.

# License

editenv is released under the MIT License:

See [LICENSE.txt](./LICENSE.txt) for a full copy of the license text.
