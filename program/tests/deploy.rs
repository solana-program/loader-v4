#![cfg(feature = "test-sbf")]

mod common;

use {
    common::{loader_v4_state_account, setup},
    mollusk_svm::result::Check,
    solana_loader_v4_program::{
        instruction::deploy,
        state::{LoaderV4State, LoaderV4Status, DEPLOYMENT_COOLDOWN_IN_SLOTS},
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
    let elf = &[4; 1_500];

    // Incorrect owner.
    let mut program_account = loader_v4_state_account(&state, elf);
    program_account.set_owner(Pubkey::new_unique());

    mollusk.process_and_validate_instruction(
        &deploy(&program, &authority, None),
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
        &deploy(&program, &authority, None),
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
    let elf = &[4; 1_500];

    let mut instruction = deploy(&program, &authority, None);
    instruction.accounts[0].is_writable = false;

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (program, loader_v4_state_account(&state, elf)),
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
    let elf = &[4; 1_500];

    let mut instruction = deploy(&program, &authority, None);
    instruction.accounts[1].is_signer = false;

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (program, loader_v4_state_account(&state, elf)),
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
    let elf = &[4; 1_500];

    mollusk.process_and_validate_instruction(
        &deploy(&program, &authority, None),
        &[
            (program, loader_v4_state_account(&state, elf)),
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
    let elf = &[4; 1_500];

    mollusk.process_and_validate_instruction(
        &deploy(&program, &authority, None),
        &[
            (program, loader_v4_state_account(&state, elf)),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::Immutable)],
    );
}

#[test]
fn fail_program_deployed_in_same_slot() {
    let mut mollusk = setup();
    mollusk.warp_to_slot(1);

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 1, // Deployed in the same slot.
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };
    let elf = &[4; 1_500];

    mollusk.process_and_validate_instruction(
        &deploy(&program, &authority, None),
        &[
            (program, loader_v4_state_account(&state, elf)),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
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
    let elf = &[4; 1_500];

    mollusk.process_and_validate_instruction(
        &deploy(&program, &authority, None),
        &[
            (program, loader_v4_state_account(&state, elf)),
            (authority, AccountSharedData::default()),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn success() {
    let mut mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0, // Not deployed yet.
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };
    let elf = &[4; 1_500];

    // First deploy for the first time.
    let check_data = {
        let mut data = vec![0; LoaderV4State::program_data_offset()];
        {
            *LoaderV4State::unpack_mut(&mut data).unwrap() = LoaderV4State {
                slot: 0,
                authority_address_or_next_version: authority,
                status: LoaderV4Status::Deployed,
            };
        }
        data.extend_from_slice(elf);
        data
    };

    let result = mollusk.process_and_validate_instruction(
        &deploy(&program, &authority, None),
        &[
            (program, loader_v4_state_account(&state, elf)),
            (authority, AccountSharedData::default()),
        ],
        &[
            Check::success(),
            Check::account(&program).data(&check_data).build(),
        ],
    );

    // Now deploy again to upgrade.
    mollusk.warp_to_slot(DEPLOYMENT_COOLDOWN_IN_SLOTS); // Tehe, can't do this in program-test :)

    // Change the program to `Retracted` in-line.
    let mut program_account = result.get_account(&program).unwrap().clone();
    program_account.data_as_mut_slice()[40] = LoaderV4Status::Retracted as u8;

    let check_data = {
        let mut data = vec![0; LoaderV4State::program_data_offset()];
        {
            *LoaderV4State::unpack_mut(&mut data).unwrap() = LoaderV4State {
                slot: mollusk.sysvars.clock.slot,
                authority_address_or_next_version: authority,
                status: LoaderV4Status::Deployed,
            };
        }
        data.extend_from_slice(elf);
        data
    };

    mollusk.process_and_validate_instruction(
        &deploy(&program, &authority, None),
        &[
            (program, program_account),
            (authority, AccountSharedData::default()),
        ],
        &[
            Check::success(),
            Check::account(&program).data(&check_data).build(),
        ],
    );
}

#[test]
fn fail_source_program_not_retracted() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let source = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };

    let source_state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Deployed, // Not retracted.
    };
    let source_elf = &[8; 1_500];

    mollusk.process_and_validate_instruction(
        &deploy(&program, &authority, Some(&source)),
        &[
            (program, loader_v4_state_account(&state, &[])),
            (authority, AccountSharedData::default()),
            (source, loader_v4_state_account(&source_state, source_elf)),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_source_program_not_owned_by_loader() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let source = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };

    let source_state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };
    let source_elf = &[8; 1_500];

    // Incorrect owner.
    let mut source_account = loader_v4_state_account(&source_state, source_elf);
    source_account.set_owner(Pubkey::new_unique());

    mollusk.process_and_validate_instruction(
        &deploy(&program, &authority, Some(&source)),
        &[
            (program, loader_v4_state_account(&state, &[])),
            (authority, AccountSharedData::default()),
            (source, source_account),
        ],
        &[Check::err(ProgramError::InvalidAccountOwner)],
    );
}

#[test]
fn fail_source_program_invalid_state() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let source = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };

    // Invalid state.
    let mut source_account =
        AccountSharedData::new(100_000_000_000, 12, &solana_loader_v4_program::id());
    source_account.set_data_from_slice(&[4; 12]);

    mollusk.process_and_validate_instruction(
        &deploy(&program, &authority, Some(&source)),
        &[
            (program, loader_v4_state_account(&state, &[])),
            (authority, AccountSharedData::default()),
            (source, source_account),
        ],
        &[Check::err(ProgramError::AccountDataTooSmall)],
    );
}

#[test]
fn fail_source_program_not_writable() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let source = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };

    let source_state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };
    let source_elf = &[8; 1_500];

    let mut instruction = deploy(&program, &authority, Some(&source));
    instruction.accounts[0].is_writable = false; // Not writable.

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (program, loader_v4_state_account(&state, &[])),
            (authority, AccountSharedData::default()),
            (source, loader_v4_state_account(&source_state, source_elf)),
        ],
        &[Check::err(ProgramError::InvalidArgument)],
    );
}

#[test]
fn fail_source_authority_not_signer() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let source = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };

    let source_state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };
    let source_elf = &[8; 1_500];

    let mut instruction = deploy(&program, &authority, Some(&source));
    instruction.accounts[1].is_signer = false; // Not a signer.

    mollusk.process_and_validate_instruction(
        &instruction,
        &[
            (program, loader_v4_state_account(&state, &[])),
            (authority, AccountSharedData::default()),
            (source, loader_v4_state_account(&source_state, source_elf)),
        ],
        &[Check::err(ProgramError::MissingRequiredSignature)],
    );
}

#[test]
fn fail_source_authority_mismatch() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let source = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };

    let source_state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: Pubkey::new_unique(), // Mismatch.
        status: LoaderV4Status::Retracted,
    };
    let source_elf = &[8; 1_500];

    mollusk.process_and_validate_instruction(
        &deploy(&program, &authority, Some(&source)),
        &[
            (program, loader_v4_state_account(&state, &[])),
            (authority, AccountSharedData::default()),
            (source, loader_v4_state_account(&source_state, source_elf)),
        ],
        &[Check::err(ProgramError::IncorrectAuthority)],
    );
}

#[test]
fn fail_source_program_finalized() {
    let mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let source = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };

    let source_state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Finalized, // Finalized.
    };
    let source_elf = &[8; 1_500];

    mollusk.process_and_validate_instruction(
        &deploy(&program, &authority, Some(&source)),
        &[
            (program, loader_v4_state_account(&state, &[])),
            (authority, AccountSharedData::default()),
            (source, loader_v4_state_account(&source_state, source_elf)),
        ],
        &[Check::err(ProgramError::Immutable)],
    );
}

#[test]
fn success_source_program() {
    let mut mollusk = setup();

    let program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let source = Pubkey::new_unique();

    let state = LoaderV4State {
        slot: 0, // Not deployed yet.
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };

    let source_state = LoaderV4State {
        slot: 0,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };
    let source_elf = &[8; 1_500];

    // First deploy for the first time.
    let check_data = {
        let mut data = vec![0; LoaderV4State::program_data_offset()];
        {
            *LoaderV4State::unpack_mut(&mut data).unwrap() = LoaderV4State {
                slot: 0,
                authority_address_or_next_version: authority,
                status: LoaderV4Status::Deployed,
            };
        }
        data.extend_from_slice(source_elf);
        data
    };

    let result = mollusk.process_and_validate_instruction(
        &deploy(&program, &authority, Some(&source)),
        &[
            (program, loader_v4_state_account(&state, &[])),
            (authority, AccountSharedData::default()),
            (source, loader_v4_state_account(&source_state, source_elf)),
        ],
        &[
            Check::success(),
            Check::account(&program).data(&check_data).build(),
            Check::account(&source).data(&[]).build(),
        ],
    );

    // Now deploy again to upgrade.
    mollusk.warp_to_slot(DEPLOYMENT_COOLDOWN_IN_SLOTS); // Tehe, can't do this in program-test :)

    let source_state = LoaderV4State {
        slot: mollusk.sysvars.clock.slot,
        authority_address_or_next_version: authority,
        status: LoaderV4Status::Retracted,
    };
    let source_elf = &[9; 1_500];

    // Change the program to `Retracted` in-line.
    let mut program_account = result.get_account(&program).unwrap().clone();
    program_account.data_as_mut_slice()[40] = LoaderV4Status::Retracted as u8;

    let check_data = {
        let mut data = vec![0; LoaderV4State::program_data_offset()];
        {
            *LoaderV4State::unpack_mut(&mut data).unwrap() = LoaderV4State {
                slot: mollusk.sysvars.clock.slot,
                authority_address_or_next_version: authority,
                status: LoaderV4Status::Deployed,
            };
        }
        data.extend_from_slice(source_elf);
        data
    };

    mollusk.process_and_validate_instruction(
        &deploy(&program, &authority, Some(&source)),
        &[
            (program, program_account),
            (authority, AccountSharedData::default()),
            (source, loader_v4_state_account(&source_state, source_elf)),
        ],
        &[
            Check::success(),
            Check::account(&program).data(&check_data).build(),
            Check::account(&source).data(&[]).build(),
        ],
    );
}
