//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>

pub(crate) mod r#deploy;
pub(crate) mod r#finalize;
pub(crate) mod r#retract;
pub(crate) mod r#transfer_authority;
pub(crate) mod r#truncate;
pub(crate) mod r#write;

pub use self::{
    r#deploy::*, r#finalize::*, r#retract::*, r#transfer_authority::*, r#truncate::*, r#write::*,
};
