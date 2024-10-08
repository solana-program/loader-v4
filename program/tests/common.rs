#![allow(dead_code)]
#![cfg(feature = "test-sbf")]

use {
    mollusk_svm::Mollusk,
    solana_loader_v4_program::state::LoaderV4State,
    solana_sdk::{account::AccountSharedData, rent::Rent, system_program},
};

pub fn setup() -> Mollusk {
    Mollusk::new(&solana_loader_v4_program::id(), "solana_loader_v4_program")
}

pub fn system_account_with_lamports(lamports: u64) -> AccountSharedData {
    AccountSharedData::new(lamports, 0, &system_program::id())
}

pub fn loader_v4_state_account(
    state: &LoaderV4State,
    additional_bytes: &[u8],
) -> AccountSharedData {
    let mut data = vec![0; LoaderV4State::program_data_offset()];
    {
        *LoaderV4State::unpack_mut(&mut data).unwrap() = *state;
    }
    data.extend_from_slice(additional_bytes);

    let space = data.len();
    let lamports = Rent::default().minimum_balance(space);

    let mut account = AccountSharedData::new(lamports, space, &solana_loader_v4_program::id());
    account.set_data_from_slice(&data);

    account
}
