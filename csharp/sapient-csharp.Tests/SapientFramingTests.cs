using System;
using System.IO;
using Google.Protobuf;
using Sapient.Bindings;
using Sapient.Bindings.BsiFlex335.V2_0;
using Xunit;

namespace Sapient.Bindings.Tests;

public sealed class SapientFramingTests
{
    [Fact]
    public void EncodeFrameWritesLittleEndianLengthHeader()
    {
        SapientMessage message = CreateMessage();

        byte[] payload = message.ToByteArray();
        byte[] frame = SapientFraming.EncodeFrame(message);

        Assert.Equal(payload.Length + 4, frame.Length);
        Assert.Equal(payload.Length, checked((int)BitConverter.ToUInt32(frame, 0)));
        Assert.Equal(payload, frame[4..]);
    }

    [Fact]
    public void DecodeFrameRoundTripsMessage()
    {
        SapientMessage message = CreateMessage();
        byte[] frame = SapientFraming.EncodeFrame(message);

        SapientMessage decoded = SapientFraming.DecodeFrame(frame, SapientMessage.Parser);

        Assert.Equal(message, decoded);
    }

    [Fact]
    public void DecodeFrameRejectsShortHeader()
    {
        byte[] frame = [0x01, 0x00, 0x00];

        Assert.Throws<InvalidDataException>(() =>
            SapientFraming.DecodeFrame(frame, SapientMessage.Parser)
        );
    }

    [Fact]
    public void DecodeFrameRejectsLengthMismatch()
    {
        byte[] frame = [0x02, 0x00, 0x00, 0x00, 0x01];

        Assert.Throws<InvalidDataException>(() =>
            SapientFraming.DecodeFrame(frame, SapientMessage.Parser)
        );
    }

    [Fact]
    public void WriteFrameAndReadFrameRoundTripMessage()
    {
        SapientMessage message = CreateMessage();
        using MemoryStream stream = new();

        SapientFraming.WriteFrame(stream, message);
        stream.Position = 0;

        SapientMessage decoded = SapientFraming.ReadFrame(stream, SapientMessage.Parser);

        Assert.Equal(message, decoded);
    }

    [Fact]
    public void ReadFrameRejectsTruncatedPayload()
    {
        SapientMessage message = CreateMessage();
        byte[] frame = SapientFraming.EncodeFrame(message);
        using MemoryStream stream = new(frame[..^1]);

        Assert.Throws<EndOfStreamException>(() =>
            SapientFraming.ReadFrame(stream, SapientMessage.Parser)
        );
    }

    private static SapientMessage CreateMessage()
    {
        return new SapientMessage
        {
            NodeId = "550e8400-e29b-41d4-a716-446655440000",
            AdditionalInformation = "framing-test",
        };
    }
}
