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

- [ ] Verify
    - [ ] Implement
    - [ ] Adjust README
- [ ] Verify templates
    - [ ] Implement
    - [ ] Adjust README
- [ ] diff
    - [ ] Find all differences
    - [ ] Delete keys not available in base
    - [ ] Insert empty Translation when key exists in base but not source
        - [ ] Output summary to output when defined and fix is on
    - [ ] Adjust README
- [ ] sort
    - [ ] Order by base ordering (order is already attached to the Translation)
    - [ ] Adjust README
- [ ] Add usage examples
