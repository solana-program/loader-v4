//! Solana BPF Loader V4 (Upgradeable) program.
#![allow(unexpected_cfgs)]

#[cfg(all(target_os = "solana", feature = "bpf-entrypoint"))]
mod entrypoint;
pub mod instruction;
pub mod processor;
pub mod state;

// [CORE BPF]: Unfortunately, the runtime still depends pretty heavily on this
// program ID hard-coded, so we can't test with it just yet.
// solana_program::declare_id!("LoaderV411111111111111111111111111111111111");
solana_program::declare_id!("CoreBPFLoaderV41111111111111111111111111111");
