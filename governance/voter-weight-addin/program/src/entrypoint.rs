//! Program entrypoint
#![cfg(all(target_arch = "bpf", not(feature = "no-entrypoint")))]

use crate::{error::VoterWeightAddinError, processor};
use paychains_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    program_error::PrintProgramError, pubkey::Pubkey,
};

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = processor::process_instruction(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<VoterWeightAddinError>();
        return Err(error);
    }
    Ok(())
}
