[![Workflow Status](https://github.com/enarx/endicon/workflows/test/badge.svg)](https://github.com/enarx/endicon/actions?query=workflow%3A%22test%22)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/enarx/endicon.svg)](https://isitmaintained.com/project/enarx/endicon "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/enarx/endicon.svg)](https://isitmaintained.com/project/enarx/endicon "Percentage of issues still open")
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# endicon

Implements endianness encodings using the `codicon` traits.

See the `codicon` crate for details.

## Examples

```rust
use endicon::Endianness;
use codicon::Encoder;

let mut bytes = [0u8; 2];
let little = [1u8, 0u8];
let big = [0u8, 1u8];

1u16.encode(&mut bytes.as_mut(), Endianness::Little).unwrap();
assert_eq!(bytes, little);

1u16.encode(&mut bytes.as_mut(), Endianness::Big).unwrap();
assert_eq!(bytes, big);
```

License: Apache-2.0
