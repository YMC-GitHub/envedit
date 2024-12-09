# envedit

_get/set os environment variables in rust._

# Contents

1. [Usage](#usage)
   - [Examples](#examples)
2. [Dependencies](#Dependencies)
3. [License](#license)

# Usage

Build envedit with `cargo build --release` and move the executable from
`target/release/envedit(.exe)` to a directory with environment access. After this
you can use envedit from the command line:

```shell
envedit set key value
envedit get key
```

_Note_: if you are in china and using [scoop](https://github.com/ScoopInstaller/Scoop) + my [scoop_bucket_cn](https://github.com/ymc-github/scoop_bucket_cn). it may be good for you:

```powershell
# use my bucket of scoop for China and name it as zero
scoop bucket add zero https://github.com/ymc-github/scoop_bucket_cn

# install envedit from bucket named zero
scoop install zero/envedit
```

## Examples

set "OLLAMA_MODELS" to "G:\data\LLM":

```shell
envedit set OLLAMA_MODELS "G:\data\LLM"
```

to get "OLLAMA_MODELS" 's value

```shell
envedit get OLLAMA_MODELS
```

# Dependencies

envedit uses the following libraries:

- ~~[clap](https://crates.io/crates/clap) - Command line argument parsing.~~
- [winreg](https://crates.io/crates/winreg) - Rust bindings to MS Windows Registry API.

# Build-Dependencies

- [winres](https://crates.io/crates/winres) - Create and set windows icons and metadata for executables.

# License

envedit is released under the MIT License:

See [LICENSE.txt](./LICENSE.txt) for a full copy of the license text.
