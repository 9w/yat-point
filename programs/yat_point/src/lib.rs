use std::str::FromStr;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    stake::program::ID as STAKE_PROGRAM_ID,
    program::invoke,
    pubkey::Pubkey,
    system_instruction,
    stake::{instruction as stake_instruction, state as stake_state},
};
use anchor_spl::token::{self, Mint, MintTo, Burn, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("Cm3ERP2QqMthfVuEfr9x1G1iEKuE8BCC5UwzfgjowvJn");

#[program]
pub mod yat_point {
    use super::*;

    pub fn stake(ctx: Context<Stake>, lamports: u64) -> Result<()> {
        let rent = Rent::get()?;
        let stake_space = 200;
        let stake_rent = rent.minimum_balance(stake_space);
        let total_lamports = lamports + stake_rent;

        // 1. Stake account 생성
        invoke(
            &system_instruction::create_account(
                ctx.accounts.user.key,
                ctx.accounts.stake_account.key,
                total_lamports,
                stake_space as u64,
                &STAKE_PROGRAM_ID,
            ),
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.stake_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        // 2. Stake account 초기화
        let auth = stake_state::Authorized {
            staker: *ctx.accounts.user.key,
            withdrawer: *ctx.accounts.user.key,
        };
        let lockup = stake_state::Lockup::default();

        invoke(
            &stake_instruction::initialize(
                ctx.accounts.stake_account.key,
                &auth,
                &lockup,
            ),
            &[
                ctx.accounts.stake_account.to_account_info(),
                ctx.accounts.rent.to_account_info(),
            ],
        )?;

        // 3. Stake account delegate
        invoke(
            &stake_instruction::delegate_stake(
                ctx.accounts.stake_account.key,
                ctx.accounts.user.key,
                ctx.accounts.validator_vote.key,
            ),
            &[
                ctx.accounts.stake_account.to_account_info(),
                ctx.accounts.validator_vote.to_account_info(),
                ctx.accounts.user.to_account_info(),
            ],
        )?;

        // 4. YP 토큰 민팅
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.yp_mint.to_account_info(),
                to: ctx.accounts.user_ata.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
        );
        token::mint_to(cpi_ctx, lamports)?;

        Ok(())
    }

    pub fn redeem(ctx: Context<Redeem>, amount: u64) -> Result<()> {
        // YP 소각
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.yp_mint.to_account_info(),
                from: ctx.accounts.user_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        );
        token::burn(cpi_ctx, amount)?;

        // 여기선 실제 unstake는 하지 않음. 예약 처리라고 가정.
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        // 간단히 vault에서 user에게 SOL 전송
        let lamports = **ctx.accounts.vault.lamports.borrow();
        **ctx.accounts.vault.lamports.borrow_mut() -= lamports;
        **ctx.accounts.user.lamports.borrow_mut() += lamports;

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(lamports: u64)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: stake 계정은 CPI로 처리
    #[account(mut)]
    pub stake_account: AccountInfo<'info>,

    #[account(mut)]
    pub yp_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = yp_mint,
        associated_token::authority = user
    )]
    pub user_ata: Account<'info, TokenAccount>,

    /// CHECK:
    pub mint_authority: AccountInfo<'info>,
    /// CHECK:
    pub validator_vote: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    /// CHECK: Stake program is a system program and doesn't require type-level enforcement
    pub stake_program: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub yp_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = yp_mint,
        associated_token::authority = user,
    )]
    pub user_ata: Account<'info, TokenAccount>,

    /// CHECK: vault is a system account used for SOL transfer, safe in this context
    #[account(mut)]
    pub vault: AccountInfo<'info>,

    /// CHECK:
    #[account(mut)]
    pub stake_account: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: vault is a system account used for SOL transfer, safe in this context
    #[account(mut)]
    pub vault: AccountInfo<'info>,
}
