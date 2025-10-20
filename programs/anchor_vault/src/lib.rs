use anchor_lang::prelude::*;

declare_id!("6RpYNZhk25mktpRowY71JzGsyQtRZTbxPN4n2FE1ga8w");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)
        
    }

    pub fn deposit(ctx: Context<Deposit>, amt: u64) -> Result<()> {
        ctx.accounts.deposit(amt)
    }


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


impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
// self gives access to the accounts defined in the Initialize and 
// bumps contains the PDA bump values automatically computed by Anchor.

        let rent_exempt = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());
        // 🧠 Breaking Down Rent Calculation
        // check here 🔆 


        let program_id = self.system_program.to_account_info();
// This just fetches the account info for Solana’s built-in System Program —
// because we’re about to make a CPI (Cross-Program Invocation) call to it.
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        /*
        Explanation:
        You’re setting up the two accounts involved in the transfer:
        from: the user’s wallet (who will pay)
        to: the vault PDA (who will receive lamports)
        Transfer is a struct defined in Anchor for the System Program CPI.
        It tells Anchor: “These are the accounts you’ll use when calling the system program’s transfer instruction.”

        💬 This is like filling out the “From” and “To” fields in a payment form.
        */

        let cpi_ctx = CpiContext::new(program_id, cpi_accounts);

        /*
        Explanation:
        CpiContext = a wrapper that holds:
        The program you’re invoking (program_id)
        The account mapping (cpi_accounts)
        Here, you’re creating a CPI context that will let you call Solana’s System Program to transfer lamports.
        💬 Think of this as setting up the “API request” to the System Program:
        “Hey System Program, I want to call your transfer instruction using these accounts.”
        */

        transfer(cpi_ctx, rent_exempt)?;
        /*
        Explanation:
        transfer is an Anchor helper function that executes the System Program’s Transfer Instruction.
        It will:
        Deduct rent_exempt lamports from from (the user),
        Add the same amount to to (the vault PDA).
        So the user funds the vault with just enough SOL to keep it rent-exempt —
        essentially “activating” it so the vault account can stay alive on-chain.

        💬 Think of it like this:
        the user is “funding” their vault locker with the minimum required SOL to make it active.
         */
        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;

        /*

        Explanation:
        Remember how PDAs need bumps?
        Anchor auto-calculates them, and you store them now in your VaultState account.
        This way, in future instructions (deposit, withdraw),
        you don’t need to re-derive PDAs using find_program_address() again —
        you just read these stored bumps and use them directly with create_program_address().

        💬 It’s like writing down the locker’s secret code in your vault record for future use.
         */
        Ok(())
        /*
        🔁 Full Flow Summary

        Here’s what the entire method does conceptually:

        Step	Action	                            Why
        1	Calculate rent exemption	            Find how much SOL the vault needs to stay alive
        2	Create CPI context for system transfer	Prepare to call Solana’s system program
        3	Transfer SOL from user → vault	        Fund the vault account
        4	Store PDA bumps in state	            Save compute units for future calls
        5	Finish	                                Initialization successful
         */

        /*
        🔑 Analogy

        User = Customer
        Vault = Locker
        System Program = Bank manager

        During initialize():

        The customer gives the minimum deposit to activate their locker.

        The bank manager (system program) accepts and moves that deposit.

        The bank writes down the locker number + access code (bumps) in their register (vault_state).
         */
    }
}

// -----------------deposit---------------------------

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account( 
        seeds=[b"state",user.key().as_ref()],
        bump=vault_state.state_bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds=[b"vault",vault_state.key().as_ref()],
        bump=vault_state.vault_bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let program_id = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(program_id, cpi_accounts);
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
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


// ---------------------------------------------
/*
🧠 Breaking Down Rent Calculation
  🔆 
    On Solana, every account must hold a minimum amount of SOL (lamports) to stay alive — this is called rent exemption.
    Ren::get()? → fetches the current rent configuration from the Solana runtime.
    minimum_balance(size) → calculates how much SOL (lamports) the account needs to stay rent-exempt.
    Here, size = self.vault.to_account_info().data_len() — basically the size of the vault account (which is zero for a SystemAccount). 
    ---
    self.vault gives you access to the Vault account defined in the struct.
    .to_account_info()

    2️⃣ .to_account_info()
    Anchor provides typed account wrappers (Account<'info, T>) for convenience, but sometimes you need the raw account info, for example, when you interact with Solana programs directly or do CPI calls.
    to_account_info() converts the typed Account<'info, Vault> into a raw AccountInfo struct (this is a Solana type).
    AccountInfo contains all the low-level account details like key, lamports, owner, data, etc.

    3️⃣ .data_len()
    AccountInfo has a method .data_len() which returns the length of the account’s data in bytes.
    This is often needed to calculate rent exemption, i.e., how many lamports you need to keep the account alive without it being reclaimed by the network.
*/


// --------------------------------------------
// what are typed accounts here
/*
1️⃣ What are typed accounts in Anchor?

Anchor gives you Rust structs that “wrap” a Solana account for type safety and convenience.

For example:

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub balance: u64,
}


Now, in your instruction context:

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
}


Here:

vault is a typed account: Account<'info, Vault>

Anchor knows exactly what data is inside (Vault struct with owner and balance)

You can do high-level Rust operations, like:

self.vault.balance += 100;
self.vault.owner = *some_pubkey;


Anchor automatically handles deserializing the account data into the struct for you. You don’t have to manually decode the raw bytes.

2️⃣ What is AccountInfo (raw account)?

Every account on Solana is basically:

pub struct AccountInfo {
    pub key: &Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
    pub lamports: &mut u64,
    pub data: &mut [u8],
    pub owner: &Pubkey,
    ...
}


This is low-level, untyped, just raw data.

If you want to do custom CPIs, transfers, or read raw bytes, you need AccountInfo.

3️⃣ Why to_account_info()?

Account<'info, Vault> is typed → convenient for Rust operations.

AccountInfo is raw → needed for Solana runtime operations, e.g., rent calculation, transfers, or CPIs.

So:

self.vault.to_account_info()


Converts your typed Vault account into a raw AccountInfo.

Then you can call:

self.vault.to_account_info().data_len()


Or use it in a CPI like system_program::transfer.

4️⃣ Analogy

Think of it like high-level class vs raw memory:

Typed Account (Account<T>)	Raw Account (AccountInfo)
Rust struct (Vault)	Raw memory bytes
Can do vault.balance += 1	Must manipulate data bytes manually
Safe, convenient	Flexible, low-level
Anchor deserializes automatically	You handle deserialization
*/

/*
solana runtime ko accounts raw form me cahiye hote hai
but anchor hume developer ki suvidha ke liye use simplified form that is typed form me bana deta hai
but when solana runtime needs that for ex. for rent cal, transfer, cpi
it converts back into raw form and perform operations.
*/