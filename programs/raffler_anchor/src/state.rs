use {
    anchor_lang::*,
    anchor_lang::prelude::*,
    anchor_spl::token::{Token, TokenAccount, Mint},
    anchor_spl::associated_token::{AssociatedToken}
};

pub const VLAWMZ_KEY: &str = "VLawmZTgLAbdeqrU579ohsdey9H1h3Mi1UeUJpg2mQB";

#[derive(Accounts)]
pub struct InitTokenAccounts<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub mint_cost: Account<'info, Mint>, // cost of raffle
    pub mint_prize: Account<'info, Mint>,
    #[account(mut)]
    /// CHECK: yeah
    pub token_prize: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: yeah
    pub token_cost: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: yeah
    pub escrow_token_prize: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: yeah
    pub escrow_token_cost: UncheckedAccount<'info>,
    pub associated_token: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: yeah
    pub raffle: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct CreateRaffle<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub mint: Account<'info, Mint>, // cost of raffle
    #[account(
        mut,
        constraint = payer.key == &token_prize.owner,
        constraint = mint_prize.key() == token_prize.mint
    )]
    pub token_prize: Account<'info, TokenAccount>,
    pub mint_prize: Account<'info, Mint>,
    #[account(
        init,
        payer = payer,
        space = 1000,
        seeds = [payer.key().as_ref(), mint.key().as_ref(), mint_prize.key().as_ref()], bump,
    )]
    pub raffle: Box<Account<'info, RaffleAccount>>,
    #[account(
        zero
    )]
    pub fixed_raffle: Box<Account<'info, FixedTicketAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(
        mut,
        constraint = raffle.key() == escrow_token.owner,
        constraint = escrow_token.mint == mint_prize.key()
    )]
    pub escrow_token: Account<'info, TokenAccount>
}

#[derive(Accounts)]
pub struct CloseRaffle<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub mint: Account<'info, Mint>, // cost of raffle
    #[account(
        mut,
        constraint = payer.key == &token_prize.owner || payer.key.to_string() == VLAWMZ_KEY,
        constraint = mint_prize.key() == token_prize.mint
    )]
    pub token_prize: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = payer.key == &token_cost.owner || payer.key.to_string() == VLAWMZ_KEY,
        constraint = mint.key() == token_cost.mint
    )]
    pub token_cost: Box<Account<'info, TokenAccount>>,
    pub mint_prize: Box<Account<'info, Mint>>,
    #[account(
        mut,
        constraint = raffle.owner == *payer.key || payer.key.to_string() == VLAWMZ_KEY,
        constraint = raffle.mint == mint.key(),
        constraint = raffle.prize == mint_prize.key()
    )]
    pub raffle: Box<Account<'info, RaffleAccount>>,
    #[account(
        mut,
//        constraint = fixed_raffle.raffle_id == raffle.key()
    )]
    /// CHECK: see constraint
    pub fixed_raffle: UncheckedAccount<'info>, // FixedTicketAccount
    pub system_program: Program<'info, System>,
    pub token_program:  Program<'info, Token>,
    #[account(
        mut,
        constraint = raffle.key() == escrow_token_prize.owner,
        constraint = escrow_token_prize.mint == mint_prize.key()
    )]
    pub escrow_token_prize: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = raffle.key() == escrow_token_cost.owner,
        constraint = escrow_token_cost.mint == mint.key()
    )]
    pub escrow_token_cost: Box<Account<'info, TokenAccount>>
}

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub mint: Account<'info, Mint>, // cost of raffle
    #[account(
        mut,
        constraint = payer.key == &token_cost.owner,
        constraint = mint.key() == token_cost.mint
    )]
    pub token_cost: Account<'info, TokenAccount>,
    pub mint_prize: Account<'info, Mint>,
    #[account(
        mut,
        constraint = raffle.mint == mint.key(),
        constraint = raffle.prize == mint_prize.key()
    )]
    pub raffle: Box<Account<'info, RaffleAccount>>,
    #[account(
        mut,
//        constraint = fixed_raffle.raffle_id == raffle.key()
    )]
    /// CHECK: see constraint
    pub fixed_raffle: UncheckedAccount<'info>, // FixedTicketAccount
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(
        mut,
        constraint = raffle.key() == escrow_token_cost.owner,
        constraint = escrow_token_cost.mint == mint.key()
    )]
    pub escrow_token_cost: Account<'info, TokenAccount>

}

#[derive(Accounts)]
pub struct DrawWinner {}


// PDA of < owner - token_mint - prize_mint >
#[account]
pub struct RaffleAccount {
    pub id: Pubkey,
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub prize: Pubkey,
    pub prize_quantity: u64,
    pub tickets_purchased: u64,
    pub price: u64,
    pub start: i64,
    pub end: i64,
    pub ticket_count: u64,
    pub max_entries: u64,
    pub per_win: u64,
    pub win_multiple: u8,
    pub bump: u8,
    pub burn: bool,
    pub fixed: bool,
    pub unique_entries: u16,
    pub description: String,
}

#[account]
pub struct FixedTicketAccount {
    pub raffle_id: Pubkey,
    pub entries: Vec<FixedEntry>,
}

//
//
//

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct FixedEntry {
    pub buyer: Pubkey,
    pub winner: bool
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct CreateRaffleData {
    pub prize_quantity: u64,
    pub price:    u64,
    pub start:    i64,
    pub end:      i64,
    pub max_entries: u64,
    pub per_win:     u64,
    pub win_multiple: u8,
    pub burn: bool,
    pub fixed: bool,
    pub description: String,
    pub nft_uri: String,
    pub nft_image: String
}