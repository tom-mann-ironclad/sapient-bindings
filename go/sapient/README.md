# sapient-go

Go protobuf bindings and framing helpers for SAPIENT / BSI Flex 335.

This module ships checked-in Go sources generated from the SAPIENT schema
source and a small framing helper layer for stream transport.

## Usage

```go
package main

import (
	"github.com/tom-mann-ironclad/sapient-bindings/go/sapient/framing"
	v2 "github.com/tom-mann-ironclad/sapient-bindings/go/sapient/generated/bsi_flex_335_v2_0"
	"google.golang.org/protobuf/proto"
)

func main() {
	message := &v2.SapientMessage{
		NodeId: proto.String("550e8400-e29b-41d4-a716-446655440000"),
	}

	frame, err := framing.EncodeFrame(message)
	if err != nil {
		panic(err)
	}

	var decoded v2.SapientMessage
	if err := framing.DecodeFrame(frame, &decoded); err != nil {
		panic(err)
	}
}
```

## Framing

The framing convention matches the Rust, Python, and C# packages:

- 4-byte unsigned little-endian payload length
- followed by the protobuf-encoded `SapientMessage` bytes

## Licensing

This module is licensed under either MIT or Apache-2.0, at your option.

The SAPIENT `.proto` schema source from which these bindings were generated is
derived from the DSTL SAPIENT-Proto-Files repository and is separately
attributed to DSTL under Apache License 2.0.
