package io.github.tommannironclad.sapient;

import com.google.protobuf.InvalidProtocolBufferException;
import com.google.protobuf.MessageLite;
import com.google.protobuf.Parser;
import java.io.EOFException;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;

/**
 * Helpers for reading and writing SAPIENT protobuf messages using the stream
 * framing convention shared by the language bindings.
 *
 * <p>A frame is encoded as a 4-byte unsigned little-endian payload length
 * followed by the protobuf-encoded message payload.
 */
public final class SapientFraming {
    private static final int HEADER_SIZE = 4;

    private SapientFraming() {
    }

    /**
     * Encodes a protobuf message as a complete SAPIENT frame.
     *
     * @param message the protobuf message to encode
     * @return a byte array containing a 4-byte little-endian length header
     *     followed by the serialized protobuf payload
     */
    public static byte[] encodeFrame(MessageLite message) {
        byte[] payload = message.toByteArray();
        byte[] frame = new byte[HEADER_SIZE + payload.length];

        int payloadLength = payload.length;
        frame[0] = (byte) payloadLength;
        frame[1] = (byte) (payloadLength >>> 8);
        frame[2] = (byte) (payloadLength >>> 16);
        frame[3] = (byte) (payloadLength >>> 24);
        System.arraycopy(payload, 0, frame, HEADER_SIZE, payload.length);

        return frame;
    }

    /**
     * Decodes a complete SAPIENT frame into a protobuf message.
     *
     * @param frame the complete framed message bytes
     * @param parser the parser for the target protobuf message type
     * @param <T> the protobuf message type to decode
     * @return the decoded protobuf message
     * @throws InvalidProtocolBufferException if the frame is shorter than the
     *     header, the payload length does not match the header, or the payload
     *     is not a valid protobuf message
     */
    public static <T extends MessageLite> T decodeFrame(byte[] frame, Parser<T> parser)
            throws InvalidProtocolBufferException {
        if (frame.length < HEADER_SIZE) {
            throw new InvalidProtocolBufferException(
                    "Frame is shorter than the 4-byte length header.");
        }

        long expectedLength = readLength(frame);
        int payloadLength = frame.length - HEADER_SIZE;
        if (payloadLength != expectedLength) {
            throw new InvalidProtocolBufferException(
                    "Frame payload length " + payloadLength
                            + " does not match header " + expectedLength + ".");
        }

        return parser.parseFrom(frame, HEADER_SIZE, payloadLength);
    }

    /**
     * Writes one protobuf message to a stream using SAPIENT framing.
     *
     * @param stream the destination stream
     * @param message the protobuf message to write
     * @throws IOException if writing to the stream fails
     */
    public static void writeFrame(OutputStream stream, MessageLite message) throws IOException {
        stream.write(encodeFrame(message));
    }

    /**
     * Reads one SAPIENT-framed protobuf message from a stream.
     *
     * @param stream the source stream
     * @param parser the parser for the target protobuf message type
     * @param <T> the protobuf message type to decode
     * @return the decoded protobuf message
     * @throws EOFException if the stream ends before the full frame header or
     *     payload has been read
     * @throws InvalidProtocolBufferException if the header length cannot fit
     *     into a Java array, or the payload is not a valid protobuf message
     * @throws IOException if reading from the stream fails
     */
    public static <T extends MessageLite> T readFrame(InputStream stream, Parser<T> parser)
            throws IOException {
        byte[] header = readExactly(stream, HEADER_SIZE);
        long payloadLength = readLength(header);
        if (payloadLength > Integer.MAX_VALUE) {
            throw new InvalidProtocolBufferException(
                    "Frame header length " + payloadLength + " is too large.");
        }

        byte[] payload = readExactly(stream, (int) payloadLength);
        return parser.parseFrom(payload);
    }

    private static long readLength(byte[] bytes) {
        return (bytes[0] & 0xFFL)
                | ((bytes[1] & 0xFFL) << 8)
                | ((bytes[2] & 0xFFL) << 16)
                | ((bytes[3] & 0xFFL) << 24);
    }

    private static byte[] readExactly(InputStream stream, int length) throws IOException {
        byte[] buffer = new byte[length];
        int offset = 0;

        while (offset < length) {
            int read = stream.read(buffer, offset, length - offset);
            if (read == -1) {
                throw new EOFException("Unexpected end of stream while reading a SAPIENT frame.");
            }

            offset += read;
        }

        return buffer;
    }
}
