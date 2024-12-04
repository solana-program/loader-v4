//! Solana BPF Loader V4 (Upgradeable) program.
#![allow(unexpected_cfgs)]

#[cfg(all(target_os = "solana", feature = "bpf-entrypoint"))]
mod entrypoint;
pub mod instruction;
pub mod processor;
pub mod state;

solana_program::declare_id!("LoaderV411111111111111111111111111111111111");
