#![cfg(feature = "test-sbf")]

mod common;

use {
    common::{loader_v4_state_account, setup},
    mollusk_svm::result::Check,
    solana_loader_v4_program::{
        instruction::finalize,
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
    let next_version = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Deployed,
    };
    let elf = &[4; 1_500];

    let next_version_state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Deployed,
    };
    let next_version_elf = &[8; 1_500];

    // Incorrect owner.
    let mut program_account = loader_v4_state_account(&state, elf);
    program_account.set_owner(Pubkey::new_unique());

    mollusk.process_and_validate_instruction(
        &finalize(&program, &authority, &next_version),
        &[
            (program, program_account),
            (authority, AccountSharedData::default()),
            (
                next_version,
                loader_v4_state_account(&next_version_state, next_version_elf),
            ),
        ],
        &[Check::err(ProgramError::InvalidAccountOwner)],
    );
}

#[test]
fn fail_program_invalid_state() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let next_version = Pubkey::new_unique();

    let next_version_state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Deployed,
    };
    let next_version_elf = &[8; 1_500];

    // Invalid state.
    let mut program_account =
        AccountSharedData::new(100_000_000_000, 12, &solana_loader_v4_program::id());
    program_account.set_data_from_slice(&[4; 12]);

    mollusk.process_and_validate_instruction(
        &finalize(&program, &authority, &next_version),
        &[
            (program, program_account),
            (authority, AccountSharedData::default()),
            (
                next_version,
                loader_v4_state_account(&next_version_state, next_version_elf),
            ),
        ],
        &[Check::err(ProgramError::AccountDataTooSmall)],
    );
}

#[test]
fn fail_program_not_writable() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let next_version = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Deployed,
    };
    let elf = &[4; 1_500];

    let next_version_state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Deployed,
    };
    let next_version_elf = &[8; 1_500];

    let mut instruction = finalize(&program, &authority, &next_version);
    instruction.accounts[0].is_writable = false; // Not writable.

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (program, loader_v4_state_account(&state, elf)),
            (authority, AccountSharedData::default()),
            (
                next_version,
                loader_v4_state_account(&next_version_state, next_version_elf),
            ),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_authority_not_signer() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let next_version = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Deployed,
    };
    let elf = &[4; 1_500];

    let next_version_state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Deployed,
    };
    let next_version_elf = &[8; 1_500];

    let mut instruction = finalize(&program, &authority, &next_version);
    instruction.accounts[1].is_signer = false; // Not a signer.

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (program, loader_v4_state_account(&state, elf)),
            (authority, AccountSharedData::default()),
            (
                next_version,
                loader_v4_state_account(&next_version_state, next_version_elf),
            ),
        ],
        &[Check::err(ProgramError::MissingRequiredSignature)],
    );
}

#[test]
fn fail_authority_mismatch() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let next_version = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: Pubkey::new_unique(), // Mismatch.
        status: LoaderV4Status::Deployed,
    };
    let elf = &[4; 1_500];

    let next_version_state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Deployed,
    };
    let next_version_elf = &[8; 1_500];

    mollusk.process_and_validate_instruction(
        &finalize(&program, &authority, &next_version),
        &[
            (program, loader_v4_state_account(&state, elf)),
            (authority, AccountSharedData::default()),
            (
                next_version,
                loader_v4_state_account(&next_version_state, next_version_elf),
            ),
        ],
        &[Check::err(ProgramError::IncorrectAuthority)],
    );
}

#[test]
fn fail_program_finalized() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let next_version = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Finalized, // Finalized.
    };
    let elf = &[4; 1_500];

    let next_version_state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Deployed,
    };
    let next_version_elf = &[8; 1_500];

    mollusk.process_and_validate_instruction(
        &finalize(&program, &authority, &next_version),
        &[
            (program, loader_v4_state_account(&state, elf)),
            (authority, AccountSharedData::default()),
            (
                next_version,
                loader_v4_state_account(&next_version_state, next_version_elf),
            ),
        ],
        &[Check::err(ProgramError::Immutable)],
    );
}

#[test]
fn fail_program_not_deployed() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let next_version = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted, // Not deployed.
    };
    let elf = &[4; 1_500];

    let next_version_state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Deployed,
    };
    let next_version_elf = &[8; 1_500];

    mollusk.process_and_validate_instruction(
        &finalize(&program, &authority, &next_version),
        &[
            (program, loader_v4_state_account(&state, elf)),
            (authority, AccountSharedData::default()),
            (
                next_version,
                loader_v4_state_account(&next_version_state, next_version_elf),
            ),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_next_version_not_owned_by_loader() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let next_version = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Deployed,
    };
    let elf = &[4; 1_500];

    let next_version_state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: Pubkey::new_unique(), // Mismatch.
        status: LoaderV4Status::Deployed,
    };
    let next_version_elf = &[8; 1_500];

    // Incorrect owner.
    let mut next_version_account = loader_v4_state_account(&next_version_state, next_version_elf);
    next_version_account.set_owner(Pubkey::new_unique());

    mollusk.process_and_validate_instruction(
        &finalize(&program, &authority, &next_version),
        &[
            (program, loader_v4_state_account(&state, elf)),
            (authority, AccountSharedData::default()),
            (next_version, next_version_account),
        ],
        &[Check::err(ProgramError::InvalidAccountOwner)],
    );
}

#[test]
fn fail_next_version_authority_mismatch() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let next_version = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Deployed,
    };
    let elf = &[4; 1_500];

    let next_version_state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: Pubkey::new_unique(), // Mismatch.
        status: LoaderV4Status::Deployed,
    };
    let next_version_elf = &[8; 1_500];

    mollusk.process_and_validate_instruction(
        &finalize(&program, &authority, &next_version),
        &[
            (program, loader_v4_state_account(&state, elf)),
            (authority, AccountSharedData::default()),
            (
                next_version,
                loader_v4_state_account(&next_version_state, next_version_elf),
            ),
        ],
        &[Check::err(ProgramError::IncorrectAuthority)],
    );
}

#[test]
fn fail_next_version_finalized() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let next_version = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Deployed,
    };
    let elf = &[4; 1_500];

    let next_version_state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Finalized, // Finalized.
    };
    let next_version_elf = &[8; 1_500];

    mollusk.process_and_validate_instruction(
        &finalize(&program, &authority, &next_version),
        &[
            (program, loader_v4_state_account(&state, elf)),
            (authority, AccountSharedData::default()),
            (
                next_version,
                loader_v4_state_account(&next_version_state, next_version_elf),
            ),
        ],
        &[Check::err(ProgramError::Immutable)],
    );
}

#[test]
fn success() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let next_version = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Deployed,
    };
    let elf = &[4; 1_500];

    let next_version_state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Deployed,
    };
    let next_version_elf = &[8; 1_500];

    let check_data = {
        let mut data = vec![0; LoaderV4State::program_data_offset()];
        {
            *LoaderV4State::unpack_mut(&mut data).unwrap() = LoaderV4State {
                slot: 0,
                authority_address_or_next_version: next_version,
                status: LoaderV4Status::Finalized,
            };
        }
        data.extend_from_slice(elf);
        data
    };

    mollusk.process_and_validate_instruction(
        &finalize(&program, &authority, &next_version),
        &[
            (program, loader_v4_state_account(&state, elf)),
            (authority, AccountSharedData::default()),
            (
                next_version,
                loader_v4_state_account(&next_version_state, next_version_elf),
            ),
        ],
        &[
            Check::success(),
            Check::compute_units(967),
            Check::account(&program).data(&check_data).build(),
        ],
    );
}
