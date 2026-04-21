# Sapient.Bindings

C# protobuf bindings and framing helpers for SAPIENT / BSI Flex 335.

This package ships checked-in C# sources generated from the SAPIENT schema
source and a small framing helper layer for stream transport.

## Usage

```csharp
using Sapient.Bindings;
using Sapient.Bindings.BsiFlex335.V2_0;

var message = new SapientMessage
{
    NodeId = "550e8400-e29b-41d4-a716-446655440000"
};

byte[] frame = SapientFraming.EncodeFrame(message);
SapientMessage decoded = SapientFraming.DecodeFrame(frame, SapientMessage.Parser);
```

## Framing

The framing convention matches the Rust and Python packages:

- 4-byte unsigned little-endian payload length
- followed by the protobuf-encoded `SapientMessage` bytes

## Licensing

This package is licensed under either MIT or Apache-2.0, at your option.

The SAPIENT `.proto` schema source from which these bindings were generated is
derived from the DSTL SAPIENT-Proto-Files repository and is separately
attributed to DSTL under Apache License 2.0.
