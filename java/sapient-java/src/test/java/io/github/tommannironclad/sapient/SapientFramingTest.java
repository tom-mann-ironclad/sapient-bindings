package io.github.tommannironclad.sapient;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import com.google.protobuf.InvalidProtocolBufferException;
import io.github.tommannironclad.sapient.bsiflex335.v2_0.SapientMessage;
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.EOFException;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.Arrays;
import org.junit.jupiter.api.Test;

class SapientFramingTest {
    @Test
    void encodeFrameWritesLittleEndianLengthHeader() {
        SapientMessage message = createMessage();

        byte[] payload = message.toByteArray();
        byte[] frame = SapientFraming.encodeFrame(message);

        assertEquals(payload.length + 4, frame.length);
        assertEquals(
                payload.length,
                ByteBuffer.wrap(frame, 0, 4).order(ByteOrder.LITTLE_ENDIAN).getInt());
        assertArrayEquals(payload, Arrays.copyOfRange(frame, 4, frame.length));
    }

    @Test
    void decodeFrameRoundTripsMessage() throws Exception {
        SapientMessage message = createMessage();
        byte[] frame = SapientFraming.encodeFrame(message);

        SapientMessage decoded = SapientFraming.decodeFrame(frame, SapientMessage.parser());

        assertEquals(message, decoded);
    }

    @Test
    void decodeFrameRejectsShortHeader() {
        byte[] frame = {0x01, 0x00, 0x00};

        assertThrows(
                InvalidProtocolBufferException.class,
                () -> SapientFraming.decodeFrame(frame, SapientMessage.parser()));
    }

    @Test
    void decodeFrameRejectsLengthMismatch() {
        byte[] frame = {0x02, 0x00, 0x00, 0x00, 0x01};

        assertThrows(
                InvalidProtocolBufferException.class,
                () -> SapientFraming.decodeFrame(frame, SapientMessage.parser()));
    }

    @Test
    void writeFrameAndReadFrameRoundTripMessage() throws Exception {
        SapientMessage message = createMessage();
        ByteArrayOutputStream output = new ByteArrayOutputStream();

        SapientFraming.writeFrame(output, message);
        ByteArrayInputStream input = new ByteArrayInputStream(output.toByteArray());

        SapientMessage decoded = SapientFraming.readFrame(input, SapientMessage.parser());

        assertEquals(message, decoded);
    }

    @Test
    void readFrameRejectsTruncatedPayload() {
        byte[] frame = SapientFraming.encodeFrame(createMessage());
        ByteArrayInputStream input =
                new ByteArrayInputStream(Arrays.copyOf(frame, frame.length - 1));

        assertThrows(
                EOFException.class,
                () -> SapientFraming.readFrame(input, SapientMessage.parser()));
    }

    @Test
    void readFrameRejectsOversizedHeader() {
        byte[] frame = {(byte) 0xFF, (byte) 0xFF, (byte) 0xFF, (byte) 0xFF};
        ByteArrayInputStream input = new ByteArrayInputStream(frame);

        assertThrows(
                InvalidProtocolBufferException.class,
                () -> SapientFraming.readFrame(input, SapientMessage.parser()));
    }

    private static SapientMessage createMessage() {
        return SapientMessage.newBuilder()
                .setNodeId("550e8400-e29b-41d4-a716-446655440000")
                .setAdditionalInformation("framing-test")
                .build();
    }
}
