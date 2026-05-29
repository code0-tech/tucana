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

#[cfg(feature = "sagittarius_gateway")]
pub mod sagittarius_gateway {
    include!("generated/sagittarius_gateway.rs");
    include!("generated/sagittarius_gateway.serde.rs");
}

#[cfg(feature = "sagittarius_rails")]
pub mod sagittarius_rails {
    include!("generated/sagittarius_rails.rs");
    include!("generated/sagittarius_rails.serde.rs");
}

#[cfg(feature = "velorum")]
pub mod velorum {
    include!("generated/velorum.rs");
    include!("generated/velorum.serde.rs");
}
