use std::{io::Error, pin::Pin};

use bytes::{Buf, BufMut, BytesMut};
use prost::Message;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// Function to send a SAPIENT message via a TCP connection with a 4 byte, little-endian length header.
pub async fn send<S, M>(message: M, mut stream: Pin<&mut S>) -> Result<(), Error>
where
    S: AsyncWrite + AsyncRead,
    M: Message,
{
    // Serialise the message to a new buffer and check the process was successful
    let mut message_buffer = BytesMut::new();
    match message.encode(&mut message_buffer) {
        Ok(_) => {}
        Err(err) => return Err(err.into()),
    }

    // Calculate the length and create a little-endian buffer of 4 bytes
    let message_length = message_buffer.len() as u32;
    let mut length_buffer = BytesMut::with_capacity(4);
    length_buffer.put_u32_le(message_length);

    // Check the header before using it
    assert_eq!(
        length_buffer.len(),
        4,
        "Expected SAPIENT message header to be 4 bytes. Found {} bytes.",
        length_buffer.len()
    );
    let mut length_buffer_check = [0_u8; 4];
    length_buffer
        .clone()
        .copy_to_slice(&mut length_buffer_check);
    assert_eq!(
        message_length,
        u32::from_le_bytes(length_buffer_check),
        "Expected header to be {}. Found {}.",
        message_length,
        u32::from_le_bytes(length_buffer_check)
    );

    // Create the final buffer to send and check that it was sent correctly
    let final_buffer = [length_buffer, message_buffer].concat();
    let written_length = stream.write(&final_buffer).await?;
    assert_eq!(
        message_length,
        (written_length - 4) as u32,
        "Not all bytes written to stream."
    );

    Ok(())
}

/// Function to receive SAPIENT messages by reading the 4 byte header
pub async fn read<S, M>(mut stream: Pin<&mut S>) -> Result<M, Error>
where
    S: AsyncWrite + AsyncRead,
    M: Message + Default,
{
    // Read length from stream
    let mut length_buffer = [0_u8; 4];
    match stream.read_exact(&mut length_buffer).await {
        Ok(_) => {}
        Err(err) => return Err(err.into()),
    };
    let expected_message_length = u32::from_le_bytes(length_buffer) as usize;

    // Read the rest of message
    let mut message_buffer = vec![0; expected_message_length];
    match stream.read_exact(&mut message_buffer).await {
        Ok(_) => {}
        Err(err) => return Err(err.into()),
    };

    // Validate the message
    let message = match M::decode(message_buffer.as_slice()) {
        Ok(message) => message,
        Err(err) => return Err(err.into()),
    };

    // Return the message
    Ok(message)
}

#[cfg(test)]
mod util_tests {
    use std::pin::Pin;

    use bytes::BytesMut;
    use prost::Message;
    use prost_types::Timestamp;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        time::{Duration as TokioDuration, timeout},
    };

    use crate::{
        bsi_flex_335_v2_0::{
            Registration, SapientMessage,
            registration::{
                Capability, Duration, ModeDefinition, ModeParameter, NodeDefinition,
                StatusDefinition, StatusReport,
            },
            sapient_message::Content,
        },
        utils::read,
    };

    use super::send;

    const IO_TIMEOUT: TokioDuration = TokioDuration::from_millis(200);

    // Function to create a SAPIENT registration message for testing
    fn create_registration() -> Registration {
        Registration {
            name: Some("My ASM".to_string()),
            short_name: None,
            capabilities: Vec::<Capability>::new(),
            status_definition: Some(StatusDefinition {
                status_interval: Some(Duration {
                    units: Some(1),
                    value: Some(1.0),
                }),
                location_definition: None,
                coverage_definition: None,
                obscuration_definition: None,
                status_report: Vec::<StatusReport>::new(),
                field_of_view_definition: None,
            }),
            mode_definition: vec![ModeDefinition {
                mode_name: Some("Default".to_string()),
                mode_type: Some(1),
                mode_description: None,
                settle_time: None,
                maximum_latency: None,
                scan_type: Some(1),
                tracking_type: Some(1),
                duration: None,
                mode_parameter: Vec::<ModeParameter>::new(),
                detection_definition: Vec::new(),
                task: None,
            }],
            dependent_nodes: Vec::new(),
            config_data: Vec::new(),
            node_definition: Vec::<NodeDefinition>::new(),
            reporting_region: Vec::new(),
            icd_version: Some("2.0".to_string()),
        }
    }

    fn create_message() -> SapientMessage {
        SapientMessage {
            timestamp: Some(Timestamp::date_time(2023, 05, 31, 09, 00, 00).unwrap()),
            node_id: Some("5c1023ac-38ec-4c3d-9aeb-ab7fc884fe12".to_string()),
            destination_id: None,
            content: Some(Content::Registration(create_registration())),
            additional_information: None,
        }
    }

    fn encode_message_frame(message: &SapientMessage) -> Vec<u8> {
        let mut message_buffer = BytesMut::new();
        message.encode(&mut message_buffer).unwrap();

        let mut frame = Vec::with_capacity(4 + message_buffer.len());
        frame.extend_from_slice(&(message_buffer.len() as u32).to_le_bytes());
        frame.extend_from_slice(&message_buffer);
        frame
    }

    /// Unit test to check that SAPIENT messages can be sent with the correct header
    #[tokio::test]
    async fn test_send_message_header() {
        // Create a new SAPIENT message
        let message = create_message();
        let mut message_buffer = BytesMut::new();
        let _ = message.clone().encode(&mut message_buffer);
        let original_message_length = message_buffer.len() as u32;

        // Create the TCP client and send the message
        let (mut client, mut server) = tokio::io::duplex(256);
        let send_result = send(message, Pin::new(&mut client)).await;
        assert_eq!(send_result.unwrap(), (), "Error in sending message.");

        // Check that the message was sent correctly
        let mut output_buffer = vec![0_u8; 256];
        let received_data = timeout(IO_TIMEOUT, server.read(&mut output_buffer))
            .await
            .expect("timed out waiting for send() to write to the stream")
            .unwrap();
        assert!(
            received_data > 0,
            "No data was received by the 'server' side of the Duplex Buffer."
        );
        assert!(!output_buffer.is_empty(), "Output buffer is empty.");

        // Check that the length of the message is correct
        let mut length_buffer = [0_u8; 4];
        for i in 0..4 {
            length_buffer[i] = output_buffer[i];
        }
        let received_len = u32::from_le_bytes(length_buffer);
        assert_eq!(
            original_message_length, received_len,
            "The received header ({} bytes) does not match the original message length ({} bytes).",
            received_len, original_message_length
        );
        assert_eq!(length_buffer, original_message_length.to_le_bytes());

        let received_message = output_buffer[4..(received_data)].to_vec();
        assert_eq!(
            received_message.len(),
            message_buffer.len(),
            "Received message length ({} bytes) does not match the original message length ({} bytes).",
            received_message.len(),
            message_buffer.len()
        );
    }

    /// Unit test to check that SAPIENT messages can be sent and encoded correctly
    #[tokio::test]
    async fn test_send_message_content() {
        // Create a new SAPIENT message
        let message = create_message();
        let mut message_buffer = BytesMut::new();
        let _ = message.encode(&mut message_buffer);
        let original_message_length = message_buffer.len() as u32;

        // Create the TCP client and send the message
        let (mut client, mut server) = tokio::io::duplex(256);
        let send_result = send(message.clone(), Pin::new(&mut client)).await;
        assert_eq!(send_result.unwrap(), (), "Error in sending message.");

        // Check that the message was sent correctly
        let mut output_buffer = vec![0_u8; 256];
        let received_data = timeout(IO_TIMEOUT, server.read(&mut output_buffer))
            .await
            .expect("timed out waiting for send() to write to the stream")
            .unwrap();
        assert!(
            received_data > 0,
            "No data was received by the 'server' side of the Duplex Buffer."
        );
        assert!(!output_buffer.is_empty(), "Output buffer is empty.");

        // Check that the length of the message is correct
        let mut length_buffer = [0_u8; 4];
        for i in 0..4 {
            length_buffer[i] = output_buffer[i];
        }
        let received_len = u32::from_le_bytes(length_buffer);
        assert_eq!(
            original_message_length, received_len,
            "The received header ({} bytes) does not match the original message length ({} bytes).",
            received_len, original_message_length
        );

        let mut received_message = BytesMut::new();
        received_message.extend_from_slice(&output_buffer[4..(received_data)]);
        assert_eq!(
            received_message.len(),
            message_buffer.len(),
            "Received message length ({} bytes) does not match the original message length ({} bytes).",
            received_message.len(),
            message_buffer.len()
        );

        for i in 0..message_buffer.len() {
            assert_eq!(
                message_buffer[i], received_message[i],
                "Received message {} does not match the original message {} at byte {}.",
                received_message[i], message_buffer[i], i
            );
        }

        // Check that received message is the same as the original message
        let decoded_message = SapientMessage::decode(received_message).unwrap();
        assert_eq!(
            message.clone(),
            decoded_message,
            "The contents of the received message ({:?}) does not match the original message ({:?}).",
            decoded_message,
            message
        );
    }

    /// Unit test to check that SAPIENT messages can be read and decoded correctly
    #[tokio::test]
    async fn test_read_message_content() {
        // Create a new SAPIENT message
        let message = create_message();

        // Create the TCP client and send the message
        let (mut client, mut server) = tokio::io::duplex(256);
        let send_result = send(message.clone(), Pin::new(&mut server)).await;
        assert_eq!(send_result.unwrap(), (), "Error in sending message.");

        // Check that the message was sent correctly
        let received_message = timeout(IO_TIMEOUT, read(Pin::new(&mut client)))
            .await
            .expect("timed out waiting for send() to produce a readable frame")
            .unwrap();

        // Check that received message is the same as the original message
        assert_eq!(
            message.clone(),
            received_message,
            "The contents of the received message ({:?}) does not match the original message ({:?}).",
            received_message,
            message
        );
    }

    #[tokio::test]
    async fn test_read_message_rejects_truncated_payload() {
        let message = create_message();
        let frame = encode_message_frame(&message);

        let (mut client, mut server) = tokio::io::duplex(256);
        server.write_all(&frame[..frame.len() - 1]).await.unwrap();
        server.shutdown().await.unwrap();

        let error = read::<_, SapientMessage>(Pin::new(&mut client))
            .await
            .unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::UnexpectedEof);
    }

    #[tokio::test]
    async fn test_read_message_rejects_invalid_payload() {
        let invalid_payload = [0xFF_u8, 0xFF, 0xFF];
        let mut frame = Vec::with_capacity(4 + invalid_payload.len());
        frame.extend_from_slice(&(invalid_payload.len() as u32).to_le_bytes());
        frame.extend_from_slice(&invalid_payload);

        let (mut client, mut server) = tokio::io::duplex(256);
        server.write_all(&frame).await.unwrap();

        let error = read::<_, SapientMessage>(Pin::new(&mut client))
            .await
            .unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
    }
}
