use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::instructions::CounterInstructions;

pub mod instructions;

pub mod tests;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CounterAccount {
    pub counter: u32,
}

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions_data: &[u8],
) -> ProgramResult {
    msg!("Counter program entry point");

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    let instruction: CounterInstructions = CounterInstructions::unpack(instructions_data)?;

    match instruction {
        CounterInstructions::Increment(args) => {
            counter_account.counter += args.value;
        }
        CounterInstructions::Decrement(args) => {
            let (difference, is_valid) = counter_account.counter.overflowing_sub(args.value);
            counter_account.counter = if is_valid { difference } else { 0 };
        }
        CounterInstructions::Reset => {
            counter_account.counter = 0;
        }
        CounterInstructions::Update(args) => {
            counter_account.counter = args.value;
        }
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}

entrypoint!(process_instruction);
