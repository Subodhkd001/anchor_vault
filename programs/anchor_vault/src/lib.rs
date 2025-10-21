use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

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

    pub fn deposit(ctx: Context<Deposit>, amt: u64) -> Result<()> {
        ctx.accounts.deposit(amt)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amt: u64) -> Result<()> {
        ctx.accounts.withdraw(amt)
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        ctx.accounts.close()
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

    #[account(
        mut,
        seeds=[b"vault",vault_state.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(&mut self, bumps: &InitializeBumps) -> Result<()> {
        let rent_exempt = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());
        let program_id = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(program_id, cpi_accounts);
        transfer(cpi_ctx, rent_exempt)?;

        self.vault_state.vault_bump = bumps.vault;
        self.vault_state.state_bump = bumps.vault_state;
        Ok(())
    }
}

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

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(seeds=[b"state",user.key().as_ref()],bump=vault_state.state_bump)]
    pub vault_state: Account<'info, VaultState>,

    #[account(mut,seeds=[b"vault",vault_state.key().as_ref()],bump=vault_state.vault_bump)]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let program_id = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(program_id, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut,close=user,seeds=[b"state",user.key().as_ref()],bump=vault_state.state_bump)]
    pub vault_state: Account<'info, VaultState>,

    #[account(mut,seeds=[b"vault",vault_state.key().as_ref()],bump=vault_state.vault_bump)]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let program_id = self.system_program.to_account_info();
        let lamports = self.vault.lamports();
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ]];
        let cpi_ctx = CpiContext::new_with_signer(program_id, cpi_accounts, signer_seeds);
        transfer(cpi_ctx, lamports)?;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}
