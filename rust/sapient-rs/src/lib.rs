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

pub mod utils;
