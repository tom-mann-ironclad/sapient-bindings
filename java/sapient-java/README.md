# sapient-bindings (Java)

Java protobuf bindings and framing helpers for SAPIENT / BSI Flex 335.

This package ships checked-in Java sources generated from the SAPIENT schema
source and a small framing helper layer for stream transport.

## Installation

Maven:

```xml
<dependency>
  <groupId>io.github.tom-mann-ironclad</groupId>
  <artifactId>sapient-bindings</artifactId>
  <version>0.1.0</version>
</dependency>
```

Gradle:

```groovy
implementation "io.github.tom-mann-ironclad:sapient-bindings:0.1.0"
```

Requires Java 8 or newer.

## Usage

```java
import io.github.tommannironclad.sapient.SapientFraming;
import io.github.tommannironclad.sapient.bsiflex335.v2_0.SapientMessage;

SapientMessage message = SapientMessage.newBuilder()
        .setNodeId("550e8400-e29b-41d4-a716-446655440000")
        .build();

byte[] frame = SapientFraming.encodeFrame(message);
SapientMessage decoded = SapientFraming.decodeFrame(frame, SapientMessage.parser());
```

## Packages

- `io.github.tommannironclad.sapient.bsiflex335.v1_0`: BSI Flex 335 v1.0 messages
- `io.github.tommannironclad.sapient.bsiflex335.v2_0`: BSI Flex 335 v2.0 messages
- `io.github.tommannironclad.sapient.proto`: shared schema options

The upstream schemas declare `uk.gov.dstl.sapientmsg.*` Java packages. These
bindings are rewritten into the `io.github.tommannironclad.sapient.*`
namespace, so they can sit on the same classpath as DSTL-provided classes.

## Framing

The framing convention matches the Rust, Python, C#, and Go packages:

- 4-byte unsigned little-endian payload length
- followed by the protobuf-encoded `SapientMessage` bytes

## Licensing

This package is licensed under either MIT or Apache-2.0, at your option.

The SAPIENT `.proto` schema source from which these bindings were generated is
derived from the DSTL SAPIENT-Proto-Files repository and is separately
attributed to DSTL under Apache License 2.0.
