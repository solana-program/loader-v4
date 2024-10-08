#![cfg(feature = "test-sbf")]

mod common;

use {
    common::{loader_v4_state_account, setup},
    mollusk_svm::result::Check,
    solana_loader_v4_program::{
        instruction::truncate,
        state::{LoaderV4State, LoaderV4Status},
    },
    solana_sdk::{
        account::{AccountSharedData, ReadableAccount, WritableAccount},
        program_error::ProgramError,
        pubkey::Pubkey,
    },
};

#[test]
fn fail_initialization_program_not_owned_by_loader() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let new_size = 36;
    let rent_exempt_lamports = mollusk
        .sysvars
        .rent
        .minimum_balance(LoaderV4State::program_data_offset().saturating_add(new_size));

    // Incorrect owner.
    let mut program_account =
        AccountSharedData::new(rent_exempt_lamports, 0, &solana_loader_v4_program::id());
    program_account.set_owner(Pubkey::new_unique());

    mollusk.process_and_validate_instruction(
        &truncate(&program, &authority, None, new_size as u32),
        &[
            (program, program_account),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidAccountOwner)],
    );
}

#[test]
fn fail_initialization_program_not_writable() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let new_size = 36;
    let rent_exempt_lamports = mollusk
        .sysvars
        .rent
        .minimum_balance(LoaderV4State::program_data_offset().saturating_add(new_size));

    let mut instruction = truncate(&program, &authority, None, new_size as u32);
    instruction.accounts[0].is_writable = false; // Not writable.

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (
                program,
                AccountSharedData::new(rent_exempt_lamports, 0, &solana_loader_v4_program::id()),
            ),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_initialization_program_not_signer() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let new_size = 36;
    let rent_exempt_lamports = mollusk
        .sysvars
        .rent
        .minimum_balance(LoaderV4State::program_data_offset().saturating_add(new_size));

    let mut instruction = truncate(&program, &authority, None, new_size as u32);
    instruction.accounts[0].is_signer = false; // Not signer.

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (
                program,
                AccountSharedData::new(rent_exempt_lamports, 0, &solana_loader_v4_program::id()),
            ),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::MissingRequiredSignature)],
    );
}

#[test]
fn fail_initialization_authority_not_signer() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let new_size = 36;
    let rent_exempt_lamports = mollusk
        .sysvars
        .rent
        .minimum_balance(LoaderV4State::program_data_offset().saturating_add(new_size));

    let mut instruction = truncate(&program, &authority, None, new_size as u32);
    instruction.accounts[1].is_signer = false; // Not signer.

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (
                program,
                AccountSharedData::new(rent_exempt_lamports, 0, &solana_loader_v4_program::id()),
            ),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::MissingRequiredSignature)],
    );
}

#[test]
fn success_initialization() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let expected_state = LoaderV4State {
        slot: mollusk.sysvars.clock.slot,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };

    let uninitialized_data = &[0; 36];

    let check_data = {
        let mut data = vec![0; LoaderV4State::program_data_offset()];
        {
            *LoaderV4State::unpack_mut(&mut data).unwrap() = expected_state;
        }
        data.extend_from_slice(uninitialized_data);
        data
    };

    let new_size = uninitialized_data.len();
    let rent_exempt_lamports = mollusk
        .sysvars
        .rent
        .minimum_balance(LoaderV4State::program_data_offset().saturating_add(new_size));

    mollusk.process_and_validate_instruction(
        &truncate(&program, &authority, None, new_size as u32),
        &[
            (
                program,
                AccountSharedData::new(rent_exempt_lamports, 0, &solana_loader_v4_program::id()),
            ),
            (authority, AccountSharedData::default()),
        ],
        &[
            Check::success(),
            Check::account(&program).data(&check_data).build(),
        ],
    );
}

#[test]
fn fail_program_not_owned_by_loader() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: mollusk.sysvars.clock.slot,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };
    let uninitialized_data = &[0; 36];

    let size_to_add = 12;
    let new_size = uninitialized_data.len().saturating_add(size_to_add);

    // Incorrect owner.
    let mut program_account = loader_v4_state_account(&state, uninitialized_data);
    program_account.set_owner(Pubkey::new_unique());

    mollusk.process_and_validate_instruction(
        &truncate(&program, &authority, None, new_size as u32),
        &[
            (program, program_account),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidAccountOwner)],
    );
}

#[test]
fn fail_program_invalid_state() {
    // "Invalid state" (small data) falls through to handling initialization.
}

#[test]
fn fail_program_not_writable() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: mollusk.sysvars.clock.slot,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };
    let uninitialized_data = &[0; 36];

    let size_to_add = 12;
    let new_size = uninitialized_data.len().saturating_add(size_to_add);

    let mut instruction = truncate(&program, &authority, None, new_size as u32);
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
        slot: mollusk.sysvars.clock.slot,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };
    let uninitialized_data = &[0; 36];

    let size_to_add = 12;
    let new_size = uninitialized_data.len().saturating_add(size_to_add);

    let mut instruction = truncate(&program, &authority, None, new_size as u32);
    instruction.accounts[1].is_signer = false; // Not signer.

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
        slot: mollusk.sysvars.clock.slot,
        authority_address_or_next_version: Pubkey::new_unique(), // Mismatch.
        status: LoaderV4Status::Retracted,
    };
    let uninitialized_data = &[0; 36];

    let size_to_add = 12;
    let new_size = uninitialized_data.len().saturating_add(size_to_add);

    mollusk.process_and_validate_instruction(
        &truncate(&program, &authority, None, new_size as u32),
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
        slot: mollusk.sysvars.clock.slot,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Finalized, // Finalized.
    };
    let uninitialized_data = &[0; 36];

    let size_to_add = 12;
    let new_size = uninitialized_data.len().saturating_add(size_to_add);

    mollusk.process_and_validate_instruction(
        &truncate(&program, &authority, None, new_size as u32),
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
        slot: mollusk.sysvars.clock.slot,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Deployed, // Not retracted.
    };
    let uninitialized_data = &[0; 36];

    let size_to_add = 12;
    let new_size = uninitialized_data.len().saturating_add(size_to_add);

    mollusk.process_and_validate_instruction(
        &truncate(&program, &authority, None, new_size as u32),
        &[
            (program, loader_v4_state_account(&state, uninitialized_data)),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_program_insufficient_lamports() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: mollusk.sysvars.clock.slot,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };
    let uninitialized_data = &[0; 36];

    let size_to_add = 12;
    let new_size = uninitialized_data.len().saturating_add(size_to_add);

    mollusk.process_and_validate_instruction(
        &truncate(&program, &authority, None, new_size as u32),
        &[
            (
                program,
                loader_v4_state_account(&state, uninitialized_data), /* No additional
                                                                      * lamports. */
            ),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InsufficientFunds)],
    );
}

#[test]
fn fail_destination_not_writable() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let destination = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: mollusk.sysvars.clock.slot,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };
    let uninitialized_data = &[0; 36];

    let size_to_remove = 12;
    let new_size = uninitialized_data.len().saturating_sub(size_to_remove);

    let mut instruction = truncate(&program, &authority, Some(&destination), new_size as u32);
    instruction.accounts[2].is_writable = false; // Not writable.

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (program, loader_v4_state_account(&state, uninitialized_data)),
            (authority, AccountSharedData::default()),
            (destination, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn success() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let destination = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: mollusk.sysvars.clock.slot,
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

    // Make the program larger, costing more lamports (pre-funded).
    const SIZE_TO_ADD: usize = 1_200;
    let new_size = uninitialized_data.len().saturating_add(SIZE_TO_ADD);
    let rent_exempt_lamports = mollusk
        .sysvars
        .rent
        .minimum_balance(LoaderV4State::program_data_offset().saturating_add(new_size));

    let mut program_account = loader_v4_state_account(&state, uninitialized_data);
    program_account.set_lamports(rent_exempt_lamports);

    check_data.extend_from_slice(&[0; SIZE_TO_ADD]);

    let result = mollusk.process_and_validate_instruction(
        &truncate(&program, &authority, None, new_size as u32),
        &[
            (program, program_account),
            (authority, AccountSharedData::default()),
        ],
        &[
            Check::success(),
            Check::account(&program).data(&check_data).build(),
        ],
    );

    // Make the program smaller.
    // Ensuring excess funds are moved to the destination account.
    const SIZE_TO_REMOVE: usize = 600;
    let new_size = uninitialized_data
        .len()
        .saturating_add(SIZE_TO_ADD)
        .saturating_sub(SIZE_TO_REMOVE);
    let rent_exempt_lamports = mollusk
        .sysvars
        .rent
        .minimum_balance(LoaderV4State::program_data_offset().saturating_add(new_size));

    let program_account = result.get_account(&program).unwrap().clone();
    let expected_destination_lamports = program_account
        .lamports()
        .saturating_sub(rent_exempt_lamports);

    check_data.truncate(LoaderV4State::program_data_offset().saturating_add(new_size));

    mollusk.process_and_validate_instruction(
        &truncate(&program, &authority, Some(&destination), new_size as u32),
        &[
            (program, program_account),
            (authority, AccountSharedData::default()),
            (destination, AccountSharedData::default()),
        ],
        &[
            Check::success(),
            Check::account(&program).data(&check_data).build(),
            Check::account(&destination)
                .lamports(expected_destination_lamports)
                .build(),
        ],
    );
}
