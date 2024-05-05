#[cfg(not(feature = "library"))]
pub mod contract;
#[cfg(not(feature = "library"))]
pub mod execute;
pub mod msg;
#[cfg(not(feature = "library"))]
pub mod query;
pub mod state;
