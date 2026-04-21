using System;
using System.IO;
using Google.Protobuf;

namespace Sapient.Bindings;

/// <summary>
/// Helpers for reading and writing SAPIENT protobuf messages using the stream
/// framing convention shared by the language bindings.
/// </summary>
/// <remarks>
/// A frame is encoded as a 4-byte unsigned little-endian payload length followed
/// by the protobuf-encoded message payload.
/// </remarks>
public static class SapientFraming
{
    private const int HeaderSize = 4;

    /// <summary>
    /// Encodes a protobuf message as a complete SAPIENT frame.
    /// </summary>
    /// <param name="message">The protobuf message to encode.</param>
    /// <returns>
    /// A byte array containing a 4-byte little-endian length header followed by
    /// the serialized protobuf payload.
    /// </returns>
    /// <exception cref="OverflowException">
    /// Thrown if the serialized payload length cannot fit in an unsigned
    /// 32-bit frame header.
    /// </exception>
    public static byte[] EncodeFrame(IMessage message)
    {
        byte[] payload = message.ToByteArray();
        byte[] frame = new byte[HeaderSize + payload.Length];

        uint payloadLength = checked((uint)payload.Length);
        frame[0] = (byte)payloadLength;
        frame[1] = (byte)(payloadLength >> 8);
        frame[2] = (byte)(payloadLength >> 16);
        frame[3] = (byte)(payloadLength >> 24);
        Buffer.BlockCopy(payload, 0, frame, HeaderSize, payload.Length);

        return frame;
    }

    /// <summary>
    /// Decodes a complete SAPIENT frame into a protobuf message.
    /// </summary>
    /// <typeparam name="T">The protobuf message type to decode.</typeparam>
    /// <param name="frame">The complete framed message bytes.</param>
    /// <param name="parser">The parser for the target protobuf message type.</param>
    /// <returns>The decoded protobuf message.</returns>
    /// <exception cref="InvalidDataException">
    /// Thrown when the frame is shorter than the header or when the payload
    /// length does not match the header.
    /// </exception>
    public static T DecodeFrame<T>(byte[] frame, MessageParser<T> parser)
        where T : IMessage<T>
    {
        if (frame.Length < HeaderSize)
        {
            throw new InvalidDataException("Frame is shorter than the 4-byte length header.");
        }

        uint expectedLength =
            frame[0]
            | ((uint)frame[1] << 8)
            | ((uint)frame[2] << 16)
            | ((uint)frame[3] << 24);

        int payloadLength = frame.Length - HeaderSize;
        if (payloadLength != expectedLength)
        {
            throw new InvalidDataException(
                $"Frame payload length {payloadLength} does not match header {expectedLength}."
            );
        }

        byte[] payload = new byte[payloadLength];
        Buffer.BlockCopy(frame, HeaderSize, payload, 0, payloadLength);
        return parser.ParseFrom(payload);
    }

    /// <summary>
    /// Writes one protobuf message to a stream using SAPIENT framing.
    /// </summary>
    /// <param name="stream">The destination stream.</param>
    /// <param name="message">The protobuf message to write.</param>
    public static void WriteFrame(Stream stream, IMessage message)
    {
        byte[] frame = EncodeFrame(message);
        stream.Write(frame, 0, frame.Length);
    }

    /// <summary>
    /// Reads one SAPIENT-framed protobuf message from a stream.
    /// </summary>
    /// <typeparam name="T">The protobuf message type to decode.</typeparam>
    /// <param name="stream">The source stream.</param>
    /// <param name="parser">The parser for the target protobuf message type.</param>
    /// <returns>The decoded protobuf message.</returns>
    /// <exception cref="EndOfStreamException">
    /// Thrown if the stream ends before the full frame header or payload has
    /// been read.
    /// </exception>
    /// <exception cref="OverflowException">
    /// Thrown if the frame header length cannot fit into an <see cref="int"/>
    /// for buffer allocation.
    /// </exception>
    public static T ReadFrame<T>(Stream stream, MessageParser<T> parser)
        where T : IMessage<T>
    {
        byte[] header = ReadExactly(stream, HeaderSize);
        uint payloadLength =
            header[0]
            | ((uint)header[1] << 8)
            | ((uint)header[2] << 16)
            | ((uint)header[3] << 24);

        byte[] payload = ReadExactly(stream, checked((int)payloadLength));
        return parser.ParseFrom(payload);
    }

    private static byte[] ReadExactly(Stream stream, int length)
    {
        byte[] buffer = new byte[length];
        int offset = 0;

        while (offset < length)
        {
            int read = stream.Read(buffer, offset, length - offset);
            if (read == 0)
            {
                throw new EndOfStreamException("Unexpected end of stream while reading a SAPIENT frame.");
            }

            offset += read;
        }

        return buffer;
    }
}
