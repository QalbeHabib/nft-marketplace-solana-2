use crate::state::*;
use anchor_lang::prelude::*;

pub fn initialize_program_state(
    ctx: Context<InitializeProgramState>,
    minting_price: u64,
) -> Result<()> {
    // ADDED: Check that the signer is the program's upgrade authority or deployer
    // This ensures only the program owner can initialize the state
    require!(
        ctx.accounts.admin.key() == ctx.accounts.expected_authority.key(),
        crate::errors::ErrorCode::UnauthorizedProgramInitialization
    );

    let program_state = &mut ctx.accounts.program_state;
    program_state.admin = ctx.accounts.admin.key();
    program_state.minting_price = minting_price;

    msg!(
        "Program state initialized by authorized admin: {}",
        ctx.accounts.admin.key()
    );
    msg!(
        "Program state initialized with minting price: {} lamports",
        minting_price
    );

    // Emit event for initialization
    emit!(ProgramStateInitialized {
        admin: ctx.accounts.admin.key(),
        minting_price,
    });

    Ok(())
}
