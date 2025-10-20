use anchor_lang::prelude::*;

declare_id!("6RpYNZhk25mktpRowY71JzGsyQtRZTbxPN4n2FE1ga8w");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    // pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer=user,
        space=VaultState::DISCRIMINATOR.len()+VaultState::INIT_SPACE,
        seeds=[b"state",user.key().as_ref()],   
        bump
    )]
    pub vault_state: Account<'info, VaultState>,
    // vault_state is the account that holds the metadata about the user's vault
    // and vault is the account that actually holds the sol

    #[account(
        mut,
        seeds=[b"vault",vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    // here we are just deriving/referencing it not creating it
    // not initializing it
    // It’s a SystemAccount (i.e., a regular Solana account that can hold SOL).
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    // pub amount: u64,
    pub vault_bump: u8,  // this is bump for vault pda
    pub state_bump: u8,  // this is bump for state pda
}

// we store bumps to save compute units
// we need not derive it again and again





// --------------------------------------------



/*

🧠 TL;DR — Why Bump Storage Saves Compute
Concept	                Description
What bump is	        A 1-byte value making a PDA valid
Why we store it	        To avoid recomputing PDA each time
What it saves	        Costly SHA-256 hashing loops (find_program_address)
How much it saves	    10–20× fewer compute units
Effect	                More efficient program, cheaper + faster execution4

Step 5: Think of It Like This
- Imagine you have to find a door’s key each time you enter your house.
- Without storing bump:
    - Every time, you try 256 keys until one fits (expensive and slow).
- With stored bump:
    - You remember which key fits, and next time just use that one directly.
    - Same door, same security, but far less effort.
*/


/*
// vault account analogy
- Imagine you run a bank (the program):
- Each customer (user) gets a vault locker (the vault SystemAccount).
- You (the bank) hold the master key (program authority).
- The bank also keeps a record card (the vault_state account) with:
- locker number (PDA seeds)
- key code (bump)
- customer name (user key)
- When the customer deposits/withdraws, the bank (program) uses that info to unlock their locker safely.
*/

/*
The vault is not initialized because:
It’s a SystemAccount, not a program-owned account.
We don’t need to store data, only SOL.
Anchor’s init is for program-owned accounts (Account<'info, T>), not for SystemAccount.
We may create it later during deposit, saving rent and compute.
It’s included only to declare seeds and link it to vault_state.
*/