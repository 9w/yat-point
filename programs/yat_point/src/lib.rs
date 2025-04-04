use std::str::FromStr;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("Cm3ERP2QqMthfVuEfr9x1G1iEKuE8BCC5UwzfgjowvJn");

#[program]
pub mod yat_point {
    use super::*;

    pub fn claim_yp(ctx: Context<ClaimYP>) -> Result<()> {
        let allowed_address = Pubkey::from_str("허용할_지갑주소").unwrap();
        require!(
            ctx.accounts.user.key() == allowed_address,
            YPError::NotAllowed
        );

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.yp_mint.to_account_info(),
                to: ctx.accounts.user_ata.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
        );

        token::mint_to(cpi_ctx, 1_000_000_000)?; // 1,000 YP with 9 decimals
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ClaimYP<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub yp_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = yp_mint,
        associated_token::authority = user
    )]
    pub user_ata: Account<'info, TokenAccount>,

    /// CHECK: 추후 PDA로 대체 가능
    pub mint_authority: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[error_code]
pub enum YPError {
    #[msg("이 주소는 YP를 받을 수 없습니다.")]
    NotAllowed,
}