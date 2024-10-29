#![cfg(feature = "test-sbf")]

mod common;

use {
    common::{loader_v4_state_account, setup},
    mollusk_svm::result::Check,
    solana_loader_v4_program::{
        instruction::write,
        state::{LoaderV4State, LoaderV4Status},
    },
    solana_sdk::{
        account::{AccountSharedData, WritableAccount},
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

#[test]
fn fail_program_not_owned_by_loader() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };
    let uninitialized_data = &[0; 36];

    // Incorrect owner.
    let mut program_account = loader_v4_state_account(&state, uninitialized_data);
    program_account.set_owner(Pubkey::new_unique());

    mollusk.process_and_validate_instruction(
        &write(&program, &authority, 0, vec![4; 12]),
        &[
            (program, program_account),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidAccountOwner)],
    );
}

#[test]
fn fail_program_invalid_state() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    // Invalid state.
    let mut program_account =
        AccountSharedData::new(100_000_000_000, 12, &solana_loader_v4_program::id());
    program_account.set_data_from_slice(&[4; 12]);

    mollusk.process_and_validate_instruction(
        &write(&program, &authority, 0, vec![4; 12]),
        &[
            (program, program_account),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::AccountDataTooSmall)],
    );
}

#[test]
fn fail_program_not_writable() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };
    let uninitialized_data = &[0; 36];

    let mut instruction = write(&program, &authority, 0, vec![4; 12]);
    instruction.accounts[0].is_writable = false; // Not writable.

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (program, loader_v4_state_account(&state, uninitialized_data)),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_authority_not_signer() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };
    let uninitialized_data = &[0; 36];

    let mut instruction = write(&program, &authority, 0, vec![4; 12]);
    instruction.accounts[1].is_signer = false; // Not a signer.

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (program, loader_v4_state_account(&state, uninitialized_data)),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::MissingRequiredSignature)],
    );
}

#[test]
fn fail_authority_mismatch() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: Pubkey::new_unique(), // Mismatch.
        status: LoaderV4Status::Retracted,
    };
    let uninitialized_data = &[0; 36];

    mollusk.process_and_validate_instruction(
        &write(&program, &authority, 0, vec![4; 12]),
        &[
            (program, loader_v4_state_account(&state, uninitialized_data)),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::IncorrectAuthority)],
    );
}

#[test]
fn fail_program_finalized() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Finalized, // Finalized.
    };
    let uninitialized_data = &[0; 36];

    mollusk.process_and_validate_instruction(
        &write(&program, &authority, 0, vec![4; 12]),
        &[
            (program, loader_v4_state_account(&state, uninitialized_data)),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::Immutable)],
    );
}

#[test]
fn fail_program_not_retracted() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Deployed, // Not retracted.
    };
    let uninitialized_data = &[0; 36];

    mollusk.process_and_validate_instruction(
        &write(&program, &authority, 0, vec![4; 12]),
        &[
            (program, loader_v4_state_account(&state, uninitialized_data)),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn success() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };
    let uninitialized_data = &[0; 36];

    let mut check_data = {
        let mut data = vec![0; LoaderV4State::program_data_offset()];
        {
            *LoaderV4State::unpack_mut(&mut data).unwrap() = state;
        }
        data.extend_from_slice(uninitialized_data);
        data
    };

    // Write successfully.
    let offset = 0;
    let bytes = vec![4; 12];
    check_data[LoaderV4State::program_data_offset()
        ..LoaderV4State::program_data_offset().saturating_add(12)]
        .copy_from_slice(&bytes);

    let result = mollusk.process_and_validate_instruction(
        &write(&program, &authority, offset, bytes),
        &[
            (program, loader_v4_state_account(&state, uninitialized_data)),
            (authority, AccountSharedData::default()),
        ],
        &[
            Check::success(),
            Check::compute_units(1_038),
            Check::account(&program).data(&check_data).build(),
        ],
    );

    // Do it again.
    let offset = 12;
    let bytes = vec![8; 24];
    check_data[LoaderV4State::program_data_offset().saturating_add(12)..].copy_from_slice(&bytes);

    mollusk.process_and_validate_instruction(
        &write(&program, &authority, offset, bytes),
        &[
            (program, result.get_account(&program).unwrap().clone()),
            (authority, AccountSharedData::default()),
        ],
        &[
            Check::success(),
            Check::compute_units(1_192),
            Check::account(&program).data(&check_data).build(),
        ],
    );
}
