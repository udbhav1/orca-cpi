use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod orcatest {
    use super::*;

    pub fn open_position(
        ctx: Context<OpenPositionCPI>,
        position_bump: u8,
        tick_lower_index: i32,
        tick_upper_index: i32
    ) -> ProgramResult {
        let cpi_program = ctx.accounts.whirlpool_program.to_account_info();
        let cpi_accounts = whirlpool::cpi::accounts::OpenPosition {
            funder: ctx.accounts.funder.to_account_info(),
            owner: ctx.accounts.owner.to_account_info(),
            position: ctx.accounts.position.to_account_info(),
            position_mint: ctx.accounts.position_mint.to_account_info(),
            position_token_account: ctx.accounts.position_token_account.to_account_info(),
            whirlpool: ctx.accounts.whirlpool.to_account_info(),

            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
            associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
        };
        let bumps = whirlpool::state::OpenPositionBumps {
            position_bump: position_bump
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        whirlpool::cpi::open_position(cpi_ctx, bumps, tick_lower_index, tick_upper_index)?;

        Ok(())
    }

    pub fn initialize_tick_array(
        ctx: Context<InitializeTickArrayCPI>,
        start_tick_index: i32
    ) -> ProgramResult {
        let cpi_program = ctx.accounts.whirlpool_program.to_account_info();
        let cpi_accounts = whirlpool::cpi::accounts::InitializeTickArray {
            whirlpool: ctx.accounts.whirlpool.to_account_info(),
            funder: ctx.accounts.funder.to_account_info(),
            tick_array: ctx.accounts.tick_array.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        whirlpool::cpi::initialize_tick_array(cpi_ctx, start_tick_index)?;

        Ok(())
    }

    pub fn increase_liquidity(
        ctx: Context<ModifyLiquidityCPI>,
        liquidity_amount: u128,
        token_max_a: u64,
        token_max_b: u64
    ) -> ProgramResult {
        let cpi_program = ctx.accounts.whirlpool_program.to_account_info();
        let cpi_accounts = whirlpool::cpi::accounts::ModifyLiquidity {
            whirlpool: ctx.accounts.whirlpool.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            position_authority: ctx.accounts.position_authority.to_account_info(),
            position: ctx.accounts.position.to_account_info(),
            position_token_account: ctx.accounts.position_token_account.to_account_info(),
            token_owner_account_a: ctx.accounts.token_owner_account_a.to_account_info(),
            token_owner_account_b: ctx.accounts.token_owner_account_b.to_account_info(),
            token_vault_a: ctx.accounts.token_vault_a.to_account_info(),
            token_vault_b: ctx.accounts.token_vault_b.to_account_info(),
            tick_array_lower: ctx.accounts.tick_array_lower.to_account_info(),
            tick_array_upper: ctx.accounts.tick_array_upper.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        whirlpool::cpi::increase_liquidity(cpi_ctx, liquidity_amount, token_max_a, token_max_b)?;

        Ok(())
    }

    pub fn decrease_liquidity(
        ctx: Context<ModifyLiquidityCPI>,
        liquidity_amount: u128,
        token_min_a: u64,
        token_min_b: u64
    ) -> ProgramResult {
        let cpi_program = ctx.accounts.whirlpool_program.to_account_info();
        let cpi_accounts = whirlpool::cpi::accounts::ModifyLiquidity {
            whirlpool: ctx.accounts.whirlpool.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            position_authority: ctx.accounts.position_authority.to_account_info(),
            position: ctx.accounts.position.to_account_info(),
            position_token_account: ctx.accounts.position_token_account.to_account_info(),
            token_owner_account_a: ctx.accounts.token_owner_account_a.to_account_info(),
            token_owner_account_b: ctx.accounts.token_owner_account_b.to_account_info(),
            token_vault_a: ctx.accounts.token_vault_a.to_account_info(),
            token_vault_b: ctx.accounts.token_vault_b.to_account_info(),
            tick_array_lower: ctx.accounts.tick_array_lower.to_account_info(),
            tick_array_upper: ctx.accounts.tick_array_upper.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        whirlpool::cpi::decrease_liquidity(cpi_ctx, liquidity_amount, token_min_a, token_min_b)?;

        Ok(())
    }

    pub fn close_position(ctx: Context<ClosePositionCPI>) -> ProgramResult {
        let cpi_program = ctx.accounts.whirlpool_program.to_account_info();
        let cpi_accounts = whirlpool::cpi::accounts::ClosePosition {
            position_authority: ctx.accounts.position_authority.to_account_info(),
            receiver: ctx.accounts.receiver.to_account_info(),
            position: ctx.accounts.position.to_account_info(),
            position_mint: ctx.accounts.position_mint.to_account_info(),
            position_token_account: ctx.accounts.position_token_account.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        whirlpool::cpi::close_position(cpi_ctx)?;
        Ok(())
    }

    pub fn update_fees_and_rewards(ctx: Context<UpdateFeesAndRewardsCPI>) -> ProgramResult {
        let cpi_program = ctx.accounts.whirlpool_program.to_account_info();
        let cpi_accounts = whirlpool::cpi::accounts::UpdateFeesAndRewards {
            whirlpool: ctx.accounts.whirlpool.to_account_info(),
            position: ctx.accounts.position.to_account_info(),
            tick_array_lower: ctx.accounts.tick_array_lower.to_account_info(),
            tick_array_upper: ctx.accounts.tick_array_upper.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        whirlpool::cpi::update_fees_and_rewards(cpi_ctx)?;
        Ok(())
    }

    pub fn collect_fees(ctx: Context<CollectFeesCPI>) -> ProgramResult {
        let cpi_program = ctx.accounts.whirlpool_program.to_account_info();

        let cpi_accounts = whirlpool::cpi::accounts::CollectFees {
            whirlpool: ctx.accounts.whirlpool.to_account_info(),
            position_authority: ctx.accounts.position_authority.to_account_info(),
            position: ctx.accounts.position.to_account_info(),
            position_token_account: ctx.accounts.position_token_account.to_account_info(),
            token_owner_account_a: ctx.accounts.token_owner_account_a.to_account_info(),
            token_vault_a: ctx.accounts.token_vault_a.to_account_info(),
            token_owner_account_b: ctx.accounts.token_owner_account_b.to_account_info(),
            token_vault_b: ctx.accounts.token_vault_b.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        whirlpool::cpi::collect_fees(cpi_ctx)?;
        Ok(())
    }

    pub fn collect_reward(ctx: Context<CollectRewardCPI>, reward_index: u8) -> ProgramResult {
        let cpi_program = ctx.accounts.whirlpool_program.to_account_info();

        let cpi_accounts = whirlpool::cpi::accounts::CollectReward {
            whirlpool: ctx.accounts.whirlpool.to_account_info(),
            position_authority: ctx.accounts.position_authority.to_account_info(),
            position: ctx.accounts.position.to_account_info(),
            position_token_account: ctx.accounts.position_token_account.to_account_info(),
            reward_owner_account: ctx.accounts.reward_owner_account.to_account_info(),
            reward_vault: ctx.accounts.reward_vault.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        whirlpool::cpi::collect_reward(cpi_ctx, reward_index)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct OpenPositionCPI<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(mut)]
    pub owner: UncheckedAccount<'info>,

    #[account(mut)]
    pub position: UncheckedAccount<'info>,

    #[account(mut)]
    pub position_mint: Signer<'info>,

    #[account(mut)]
    pub position_token_account: AccountInfo<'info>,

    #[account(mut)]
    pub whirlpool: Box<Account<'info, whirlpool::state::Whirlpool>>,

    pub whirlpool_program: Program<'info, whirlpool::program::Whirlpool>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct InitializeTickArrayCPI<'info> {
    pub whirlpool_program: Program<'info, whirlpool::program::Whirlpool>,

    pub whirlpool: Account<'info, whirlpool::state::Whirlpool>,

    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(mut)]
    pub tick_array: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyLiquidityCPI<'info> {
    pub whirlpool_program: Program<'info, whirlpool::program::Whirlpool>,

    #[account(mut)]
    pub whirlpool: Account<'info, whirlpool::state::Whirlpool>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,

    pub position_authority: Signer<'info>,

    #[account(mut, has_one = whirlpool)]
    pub position: Account<'info, whirlpool::state::Position>,

    #[account(
        constraint = position_token_account.mint == position.position_mint,
        constraint = position_token_account.amount == 1
    )]
    pub position_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = token_owner_account_a.mint == whirlpool.token_mint_a)]
    pub token_owner_account_a: Box<Account<'info, TokenAccount>>,
    #[account(mut, constraint = token_owner_account_b.mint == whirlpool.token_mint_b)]
    pub token_owner_account_b: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = token_vault_a.key() == whirlpool.token_vault_a)]
    pub token_vault_a: Box<Account<'info, TokenAccount>>,
    #[account(mut, constraint = token_vault_b.key() == whirlpool.token_vault_b)]
    pub token_vault_b: Box<Account<'info, TokenAccount>>,

    #[account(mut, has_one = whirlpool)]
    pub tick_array_lower: AccountLoader<'info, whirlpool::state::TickArray>,
    #[account(mut, has_one = whirlpool)]
    pub tick_array_upper: AccountLoader<'info, whirlpool::state::TickArray>,
}

#[derive(Accounts)]
pub struct ClosePositionCPI<'info> {
    pub whirlpool_program: Program<'info, whirlpool::program::Whirlpool>,

    pub position_authority: Signer<'info>,

    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,

    #[account(mut, close = receiver)]
    pub position: Account<'info, whirlpool::state::Position>,

    #[account(mut, address = position.position_mint)]
    pub position_mint: Account<'info, Mint>,

    #[account(mut,
        constraint = position_token_account.amount == 1,
        constraint = position_token_account.mint == position.position_mint)]
    pub position_token_account: Box<Account<'info, TokenAccount>>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateFeesAndRewardsCPI<'info> {
    pub whirlpool_program: Program<'info, whirlpool::program::Whirlpool>,

    #[account(mut)]
    pub whirlpool: Account<'info, whirlpool::state::Whirlpool>,

    #[account(mut, has_one = whirlpool)]
    pub position: Account<'info, whirlpool::state::Position>,

    #[account(has_one = whirlpool)]
    pub tick_array_lower: AccountLoader<'info, whirlpool::state::TickArray>,
    #[account(has_one = whirlpool)]
    pub tick_array_upper: AccountLoader<'info, whirlpool::state::TickArray>,
}

#[derive(Accounts)]
pub struct CollectFeesCPI<'info> {
    pub whirlpool_program: Program<'info, whirlpool::program::Whirlpool>,

    pub whirlpool: Box<Account<'info, whirlpool::state::Whirlpool>>,

    pub position_authority: Signer<'info>,

    #[account(mut, has_one = whirlpool)]
    pub position: Box<Account<'info, whirlpool::state::Position>>,
    #[account(
        constraint = position_token_account.mint == position.position_mint,
        constraint = position_token_account.amount == 1
    )]
    pub position_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = token_owner_account_a.mint == whirlpool.token_mint_a)]
    pub token_owner_account_a: Box<Account<'info, TokenAccount>>,
    #[account(mut, address = whirlpool.token_vault_a)]
    pub token_vault_a: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = token_owner_account_b.mint == whirlpool.token_mint_b)]
    pub token_owner_account_b: Box<Account<'info, TokenAccount>>,
    #[account(mut, address = whirlpool.token_vault_b)]
    pub token_vault_b: Box<Account<'info, TokenAccount>>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(reward_index: u8)]
pub struct CollectRewardCPI<'info> {
    pub whirlpool_program: Program<'info, whirlpool::program::Whirlpool>,

    pub whirlpool: Box<Account<'info, whirlpool::state::Whirlpool>>,

    pub position_authority: Signer<'info>,

    #[account(mut, has_one = whirlpool)]
    pub position: Box<Account<'info, whirlpool::state::Position>>,
    #[account(
        constraint = position_token_account.mint == position.position_mint,
        constraint = position_token_account.amount == 1
    )]
    pub position_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut,
        constraint = reward_owner_account.mint == whirlpool.reward_infos[reward_index as usize].mint
    )]
    pub reward_owner_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, address = whirlpool.reward_infos[reward_index as usize].vault)]
    pub reward_vault: Box<Account<'info, TokenAccount>>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
}