# Verethiel

Verethiel is a tool for verifying and fixing translation files.
The current plan is to support `json` and `{{}}` templating,
but I am considering supporting other template styles and languages with feature flags.

## Features:

- Detect missing translation keys
- Detect diverging template placeholders (`{{key}}`)
- Remove obsolete keys
- Create reports of key divergence
- Fix key divergences

## Installation

### From source

```sh
git clone git@github.com:LukasHuthOrg/verethiel.git
pushd verethiel
cargo install --path .
popd
```

## Road map

- [x] Verify
    - [x] Implement
    - [x] Adjust README
- [ ] Verify templates
    - [ ] Implement
    - [ ] Adjust README
- [ ] diff
    - [x] Find all differences
    - [ ] Delete keys not available in base
    - [ ] Insert empty Translation when key exists in base but not source
        - [ ] Output summary to output when defined and fix is on
    - [ ] Adjust README
- [x] sort
    - [x] Order by base ordering (order is already attached to the Translation)
    - [x] Adjust README
- [ ] Add usage examples
- [ ] Toggle Stream/Transaction
- [ ] Add `verethiel.toml` support for feature flags

## Usage

### Verify

This command verifies, whether the keys of the source file/s match the keys of the base file.
It flags missing keys and unknown keys.

#### Command

```sh
verethiel verify [OPTIONS] <BASE FILE> <SOURCE>
```
or
```sh
verethiel v [OPTIONS] <BASE FILE> <SOURCE>
```

#### Arguments
- *BASE FILE*: This file will be used as template what structure the other files should satisfy.
- *SOURCE*: This will be compared against the base. This can be a file or a directory.

#### Options
- *recursive*: This can be toggled when the specified source is a directory to check every subdirectory as well.
    - Usage: `--recursive` or `-r`
- *strict*: In strict mode the order of the keys is also being validated.
    - Usage: `--strict` or `-s`

### Sort

This command uses the structure of the base and sorts the keys in based like they are in the base.
Keys which are not in the base are appended below the known keys.

#### Command

```sh
verethiel sort [OPTIONS] <BASE FILE> <SOURCE>
```
or
```sh
verethiel s [OPTIONS] <BASE FILE> <SOURCE>
```

#### Arguments
- *BASE FILE*: This file will be used as template what structure the other files should satisfy.
- *SOURCE*: This will be sorted according to the structure of the base. This can be a file or a directory.

#### Options
- *recursive*: This can be toggled when the specified source is a directory to check every subdirectory as well.
    - Usage: `--recursive` or `-r`
- *strict*: In strict the sort will fail when encountering missing or unknown keys
    - Usage: `--strict` or `-s`

### Diff

This command uses the stucture of the base and compares it against every source.
It will then output what keys are missing and extra.

#### Command

```sh
verethiel diff [OPTIONS] <BASE FILE> <SOURCE>
```
or
```sh
verethiel d [OPTIONS] <BASE FILE> <SOURCE>
```

#### Arguments
- *BASE FILE*: This file will be used as template what keys the other files should have.
- *SOURCE*: This will be diffed against the base. It can be a file or a directory

#### Options
- *recursive*: This can be toggled when the source is a directory to check every subdirectory as well.
    - Usage: `--recursive` or `-r`
- *output*: With this you can specify an output file to not print the result to stdout.
    - Usage: `--output` or `-o`

## Supported Formats

Currently supported:
- JSON files
- `{{key}}` placeholder templating

Planned (via feature-flag):
- Additional template styles
- Other structured formats
