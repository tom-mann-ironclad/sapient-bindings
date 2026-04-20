# sapient-py

Python protobuf bindings and framing helpers for SAPIENT / BSI Flex 335.

This package ships checked-in Python modules generated from the SAPIENT schema
source and a small framing helper layer for stream transport.

The generated modules live under `sapient_msg`, while the framing helpers live
under `sapient_py`.

## Usage

```python
from sapient_msg.bsi_flex_335_v2_0 import sapient_message_pb2
from sapient_py.framing import encode_frame, decode_frame

message = sapient_message_pb2.SapientMessage()
message.node_id = "550e8400-e29b-41d4-a716-446655440000"

frame = encode_frame(message)
decoded = decode_frame(frame, sapient_message_pb2.SapientMessage)
```

## Framing

The framing convention matches the Rust crate:

- 4-byte unsigned little-endian payload length
- followed by the protobuf-encoded `SapientMessage` bytes

## Licensing

This package is licensed under either MIT or Apache-2.0, at your option.

The SAPIENT `.proto` schema source from which these bindings were generated is
derived from the DSTL SAPIENT-Proto-Files repository and is separately
attributed to DSTL under Apache License 2.0.
