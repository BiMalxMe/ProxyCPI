use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo}, entrypoint,entrypoint:: ProgramResult, example_mocks::solana_sdk::system_instruction, msg, program::invoke, program_error::ProgramError, pubkey::Pubkey, rent::Rent, sysvar::Sysvar
};

#[derive(BorshSerialize, BorshDeserialize)]
struct CounterState {
    count: u32,
}

#[derive(BorshSerialize, BorshDeserialize)]
enum CounterInstruction {
    Initialize,
    Double,
    Half,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CounterInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        CounterInstruction::Initialize => {
            msg!("Initializing counter");
            
            let mut iter = accounts.iter();
            let data_account = next_account_info(&mut iter)?;
            let payer = next_account_info(&mut iter)?;
            let system_program = next_account_info(&mut iter)?;
            
            // Check if payer is signer
            if !payer.is_signer {
                return Err(ProgramError::MissingRequiredSignature);
            }
            
            // Calculate space needed for CounterState
            let space = 4;
            
            // Calculate rent exemption amount
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            
            // Create the account
            let create_account_ix = system_instruction::create_account(
                payer.key,
                data_account.key,
                lamports,
                space as u64,
                program_id,
            );
            
            invoke(
                &create_account_ix,
                &[
                    payer.clone(),
                    data_account.clone(),
                    system_program.clone(),
                ],
            )?;
            
            // Initialize the account data
            let counter_state = CounterState { count: 1 };
            counter_state.serialize(&mut *data_account.data.borrow_mut())?;
        }
        CounterInstruction::Double => {
            let mut iter = accounts.iter();
            let data_account = next_account_info(&mut iter)?;
            msg!("Doubling the counter");
            let mut counter_state = CounterState::try_from_slice(&data_account.data.borrow())?;
            counter_state.count *= 2;
            counter_state.serialize(&mut *data_account.data.borrow_mut())?;
        }
        CounterInstruction::Half => {
            let mut iter = accounts.iter();
            let data_account = next_account_info(&mut iter)?;
            msg!("Halving the counter");
            let mut counter_state = CounterState::try_from_slice(&data_account.data.borrow())?;
            counter_state.count /= 2;
            counter_state.serialize(&mut *data_account.data.borrow_mut())?;
        }
    }
    Ok(())
}