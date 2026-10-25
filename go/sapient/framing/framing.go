package framing

import (
	"encoding/binary"
	"fmt"
	"io"

	"google.golang.org/protobuf/proto"
)

const headerSize = 4

// EncodeFrame encodes a protobuf message as a complete SAPIENT frame.
//
// Frames use a 4-byte unsigned little-endian payload length followed by the
// protobuf-encoded message payload.
func EncodeFrame(message proto.Message) ([]byte, error) {
	payload, err := proto.Marshal(message)
	if err != nil {
		return nil, err
	}

	frame := make([]byte, headerSize+len(payload))
	binary.LittleEndian.PutUint32(frame[:headerSize], uint32(len(payload)))
	copy(frame[headerSize:], payload)

	return frame, nil
}

// DecodeFrame decodes a complete SAPIENT frame into message.
func DecodeFrame(frame []byte, message proto.Message) error {
	if len(frame) < headerSize {
		return fmt.Errorf("frame is shorter than the %d-byte length header", headerSize)
	}

	expectedLength := binary.LittleEndian.Uint32(frame[:headerSize])
	payload := frame[headerSize:]
	if len(payload) != int(expectedLength) {
		return fmt.Errorf("frame payload length %d does not match header %d", len(payload), expectedLength)
	}

	return proto.Unmarshal(payload, message)
}

// WriteFrame writes one protobuf message to writer using SAPIENT framing.
func WriteFrame(writer io.Writer, message proto.Message) error {
	frame, err := EncodeFrame(message)
	if err != nil {
		return err
	}

	_, err = writer.Write(frame)
	return err
}

// ReadFrame reads one SAPIENT-framed protobuf message from reader.
func ReadFrame(reader io.Reader, message proto.Message) error {
	header := make([]byte, headerSize)
	if _, err := io.ReadFull(reader, header); err != nil {
		return err
	}

	payloadLength := binary.LittleEndian.Uint32(header)
	payload := make([]byte, payloadLength)
	if _, err := io.ReadFull(reader, payload); err != nil {
		return err
	}

	return proto.Unmarshal(payload, message)
}
