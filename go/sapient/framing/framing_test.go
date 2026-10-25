package framing_test

import (
	"bytes"
	"encoding/binary"
	"errors"
	"io"
	"testing"

	"github.com/tom-mann-ironclad/sapient-bindings/go/sapient/framing"
	v2 "github.com/tom-mann-ironclad/sapient-bindings/go/sapient/generated/bsi_flex_335_v2_0"
	"google.golang.org/protobuf/proto"
)

func TestEncodeFrameWritesLittleEndianLengthHeader(t *testing.T) {
	message := testMessage()
	payload, err := proto.Marshal(message)
	if err != nil {
		t.Fatal(err)
	}

	frame, err := framing.EncodeFrame(message)
	if err != nil {
		t.Fatal(err)
	}

	if got, want := len(frame), len(payload)+4; got != want {
		t.Fatalf("frame length = %d, want %d", got, want)
	}
	if got, want := binary.LittleEndian.Uint32(frame[:4]), uint32(len(payload)); got != want {
		t.Fatalf("payload length header = %d, want %d", got, want)
	}
	if !bytes.Equal(frame[4:], payload) {
		t.Fatal("frame payload does not match protobuf payload")
	}
}

func TestDecodeFrameRoundTripsMessage(t *testing.T) {
	message := testMessage()
	frame, err := framing.EncodeFrame(message)
	if err != nil {
		t.Fatal(err)
	}

	var decoded v2.SapientMessage
	if err := framing.DecodeFrame(frame, &decoded); err != nil {
		t.Fatal(err)
	}

	if !proto.Equal(message, &decoded) {
		t.Fatalf("decoded message = %v, want %v", &decoded, message)
	}
}

func TestDecodeFrameRejectsShortHeader(t *testing.T) {
	var decoded v2.SapientMessage
	err := framing.DecodeFrame([]byte{0x01, 0x00, 0x00}, &decoded)
	if err == nil {
		t.Fatal("expected an error")
	}
}

func TestDecodeFrameRejectsLengthMismatch(t *testing.T) {
	var decoded v2.SapientMessage
	err := framing.DecodeFrame([]byte{0x02, 0x00, 0x00, 0x00, 0x01}, &decoded)
	if err == nil {
		t.Fatal("expected an error")
	}
}

func TestWriteFrameAndReadFrameRoundTripMessage(t *testing.T) {
	message := testMessage()
	buffer := new(bytes.Buffer)

	if err := framing.WriteFrame(buffer, message); err != nil {
		t.Fatal(err)
	}

	var decoded v2.SapientMessage
	if err := framing.ReadFrame(buffer, &decoded); err != nil {
		t.Fatal(err)
	}

	if !proto.Equal(message, &decoded) {
		t.Fatalf("decoded message = %v, want %v", &decoded, message)
	}
}

func TestReadFrameRejectsTruncatedPayload(t *testing.T) {
	message := testMessage()
	frame, err := framing.EncodeFrame(message)
	if err != nil {
		t.Fatal(err)
	}

	var decoded v2.SapientMessage
	err = framing.ReadFrame(bytes.NewReader(frame[:len(frame)-1]), &decoded)
	if !errors.Is(err, io.ErrUnexpectedEOF) {
		t.Fatalf("error = %v, want %v", err, io.ErrUnexpectedEOF)
	}
}

func testMessage() *v2.SapientMessage {
	return &v2.SapientMessage{
		NodeId:                proto.String("550e8400-e29b-41d4-a716-446655440000"),
		AdditionalInformation: proto.String("framing-test"),
	}
}
