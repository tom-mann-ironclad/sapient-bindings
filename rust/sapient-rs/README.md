# sapient-rs

Rust bindings for the SAPIENT / BSI Flex 335 protobuf schemas.

This crate is maintained by Tom Mann, one of the Principal Authors of the
SAPIENT standard. It is not affiliated with or sponsored by DSTL.

The generated bindings are distinct from the protocol specification itself.
They are derived from schema definitions published in the
[DSTL SAPIENT-Proto-Files repository](https://github.com/dstl/SAPIENT-Proto-Files).
Those source schema files are attributed to DSTL and remain available under
Apache License 2.0.

## Features

- `v2_0` is enabled by default
- `v1_0` enables the older BSI Flex 335 v1.0 bindings
- `v1_0,v2_0` enables both schema versions in one build

## Usage

Add the crate with:

```bash
cargo add sapient-rs
```

Import the schema version you want:

```rust
use sapient_rs::bsi_flex_335_v2_0::SapientMessage;
```

The generated types implement `prost::Message`, so encoding and decoding uses
the usual `prost` APIs:

```rust
use prost::Message;
use prost_types::Timestamp;
use sapient_rs::bsi_flex_335_v2_0::{
    Registration, SapientMessage,
    sapient_message::Content,
};

fn main() {
    let msg = SapientMessage {
        timestamp: Some(Timestamp {
            seconds: 1_713_000_000,
            nanos: 0,
        }),
        node_id: Some("550e8400-e29b-41d4-a716-446655440000".to_string()),
        destination_id: None,
        content: Some(Content::Registration(Registration {
            node_definition: Vec::new(),
            icd_version: Some("BSI_Flex_335_v2.0".to_string()),
            name: Some("Example Node".to_string()),
            short_name: Some("example".to_string()),
            capabilities: Vec::new(),
            status_definition: None,
            mode_definition: Vec::new(),
            dependent_nodes: Vec::new(),
            reporting_region: Vec::new(),
            config_data: Vec::new(),
        })),
        additional_information: None,
    };

    let bytes = msg.encode_to_vec();
    let decoded = SapientMessage::decode(bytes.as_slice()).unwrap();

    println!("{decoded:?}");
}
```

## Framing

This crate also provides simple framing helpers in `sapient_rs::utils` for a
stream transport of:

- 4-byte unsigned little-endian payload length
- followed by the protobuf-encoded message bytes

## Licensing

This crate is licensed under either of:

- MIT
- Apache-2.0

at your option.

The SAPIENT `.proto` schema source from which these bindings were generated is
derived from the DSTL SAPIENT-Proto-Files repository and is separately
attributed to DSTL under Apache License 2.0.
