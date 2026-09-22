#[cfg(not(any(feature = "v1_0", feature = "v2_0")))]
compile_error!("Enable at least one schema feature (`v2_0` is enabled by default).");

pub mod sapient_msg {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/generated/sapient_msg.rs"
    ));

    #[cfg(feature = "v1_0")]
    pub mod bsi_flex_335_v1_0 {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/generated/sapient_msg.bsi_flex_335_v1_0.rs"
        ));
    }

    #[cfg(feature = "v2_0")]
    pub mod bsi_flex_335_v2_0 {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/generated/sapient_msg.bsi_flex_335_v2_0.rs"
        ));
    }
}

#[cfg(feature = "v1_0")]
pub use sapient_msg::bsi_flex_335_v1_0;
#[cfg(feature = "v2_0")]
pub use sapient_msg::bsi_flex_335_v2_0;

/// A compiled `FileDescriptorSet` (see the
/// [protobuf reflection docs](https://protobuf.dev/reference/other/#descriptor))
/// covering every `sapient_msg` package this crate generates bindings for
/// (`bsi_flex_335_v1_0` and `bsi_flex_335_v2_0`, regardless of which schema
/// features are enabled). Intended for tools that need to work with SAPIENT
/// messages generically at runtime -- for example decoding canonical
/// protobuf JSON via [`prost-reflect`](https://docs.rs/prost-reflect) -- and
/// would otherwise need their own copy of the `.proto` sources.
pub static FILE_DESCRIPTOR_SET_BYTES: &[u8] =
    include_bytes!("generated/sapient_msg.file_descriptor_set.bin");

pub mod utils;
