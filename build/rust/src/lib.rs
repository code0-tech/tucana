#[cfg(feature = "sagittarius")]
pub mod sagittarius {
    include!("generated/sagittarius.rs");
}

#[cfg(feature = "aquila")]
pub mod aquila {
    include!("generated/aquila.rs");
}

#[cfg(feature = "shared")]
pub mod shared {
    include!("generated/shared.rs");
}