"""Length-prefixed framing helpers for SAPIENT protobuf messages."""

from __future__ import annotations

from typing import BinaryIO, TypeVar

from google.protobuf.message import Message

_MessageT = TypeVar("_MessageT", bound=Message)

HEADER_SIZE = 4


def encode_frame(message: Message) -> bytes:
    """Encode a protobuf message with a 4-byte little-endian length prefix."""
    payload = message.SerializeToString()
    return len(payload).to_bytes(HEADER_SIZE, byteorder="little") + payload


def decode_frame(frame: bytes, message_type: type[_MessageT]) -> _MessageT:
    """Decode a complete length-prefixed frame into the requested message type."""
    if len(frame) < HEADER_SIZE:
        raise ValueError("frame is shorter than the 4-byte length header")

    expected_length = int.from_bytes(frame[:HEADER_SIZE], byteorder="little")
    payload = frame[HEADER_SIZE:]
    if len(payload) != expected_length:
        raise ValueError(
            f"frame payload length {len(payload)} does not match header {expected_length}"
        )

    message = message_type()
    message.ParseFromString(payload)
    return message


def read_frame(stream: BinaryIO, message_type: type[_MessageT]) -> _MessageT:
    """Read exactly one framed protobuf message from a binary stream."""
    header = stream.read(HEADER_SIZE)
    if len(header) != HEADER_SIZE:
        raise EOFError("could not read complete 4-byte frame header")

    payload_length = int.from_bytes(header, byteorder="little")
    payload = stream.read(payload_length)
    if len(payload) != payload_length:
        raise EOFError("could not read complete protobuf payload")

    message = message_type()
    message.ParseFromString(payload)
    return message


def write_frame(stream: BinaryIO, message: Message) -> None:
    """Write exactly one framed protobuf message to a binary stream."""
    stream.write(encode_frame(message))
