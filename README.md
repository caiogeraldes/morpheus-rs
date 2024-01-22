# morpheus-rs

A library and CLI for querying and parsing the Perseis Morpheus API.

## Instalation

### Ubuntu

Download the latest release.

```{bash}
sudo dpkg -i morpheus-cli_<VERSION>_amd64.deb
```

### Manual

Clone the repository

```{bash}
git clone https://github.com/caiogeraldes/morpheus-rs
```

Build it

```{bash}
cd morpheus-rs
cargo build --release
```

Copy binary to an executable path, e.g:

```{bash}
cp ./target/release/morpheus-cli $HOME/.local/bin/
```

## Usage of the CLI

### Basics

Latin words can be queried simply by using:

```{bash}
morpheus-cli "homo"
```

The CLI will try to identify if the input is Greek (both unicode or betacode encoded).
You can force the betacode conversion with the flag `-b/--betacode`

```{bash}
morpheus-cli "ἄνδρα"
> headword,headword_decl,headword_gend,stem,suff,decl,pofs,gend,num,mood,tense,voice,pers,case,dial,stemtype,morph,derivtype
> ἀνήρ,3rd,masculine,ἄνδρα,,3rd,noun,masculine,singular,,,,,accusative,,irreg_decl3,indeclform,
morpheus-cli "a)/ndra"
> headword,headword_decl,headword_gend,stem,suff,decl,pofs,gend,num,mood,tense,voice,pers,case,dial,stemtype,morph,derivtype
> ἀνήρ,3rd,masculine,ἄνδρα,,3rd,noun,masculine,singular,,,,,accusative,,irreg_decl3,indeclform,
```

The output may be converted to a prettier format by passing the flag `-p/--pretty`,
using [tv](https://github.com/alexhallam/tv):

```{bash}
morpheus-cli "ἄνδρα" -p
>   tv dim: 1 x 18
>         headword headword_decl headword_gend stem  suff decl pofs gend      num      mood tense voice pers case       dial stemtype    morph      derivtype
>   1     ἀνήρ     3rd           masculine     ἄνδρα NA   3rd  noun masculine singular NA   NA    NA    NA   accusative NA   irreg_decl3 indeclform NA
```

To keep the queried form on the CSV output, pass the flag `-q/--query`

```{bash}
morpheus-cli "ἄνδρα" -p -q
>   tv dim: 1 x 18
>         queryword headword headword_decl headword_gend stem  suff decl pofs gend      num      mood tense voice pers case       dial stemtype    morph      derivtype
>   1     ἄνδρα     ἀνήρ     3rd           masculine     ἄνδρα NA   3rd  noun masculine singular NA   NA    NA    NA   accusative NA   irreg_decl3 indeclform NA
```

#### Saving to file

Pass the argument `-o <FILE>` or `--output <FILE>` for saving the response to a file.

### Advanced

#### JSON Response

To retrieve the raw `JSON` response from the API, pass the option `-j/--json`

#### Cache

To make querying faster, the default operation caches the responses from the API
in the path `$HOME/.cache/morpheus_rs/`. You may change the path of the cache
by setting the environment variabel `MORPHEUS_CACHE_DIR`.

If needed, you may run the command with the flag `-u/--uncached` for bypassing
the caching mechanism.
