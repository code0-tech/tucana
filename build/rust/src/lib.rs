pub mod shared {
    pub mod helper;

    include!("generated/shared.rs");
    include!("generated/shared.serde.rs");
}

#[cfg(feature = "aquila")]
pub mod aquila {
    include!("generated/aquila.rs");
    include!("generated/aquila.serde.rs");
}

#[cfg(feature = "sagittarius")]
pub mod sagittarius {
    include!("generated/sagittarius.rs");
    include!("generated/sagittarius.serde.rs");
}
