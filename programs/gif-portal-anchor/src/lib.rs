use anchor_lang::prelude::*;

declare_id!("FzwXxcH2FVRaqXtXx41N8vxpKPoQY3wRPnJJ7TMjP3da");

#[program]
pub mod gif_portal_anchor {
    use anchor_lang::solana_program::entrypoint::ProgramResult;
    use super::*;

   pub fn start_stuff_off(ctx: Context<StartStuffOff>)->ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
   }
    pub fn add_gif(ctx: Context<AddGif>, gif_link:String)->ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct{
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            num_votes: 0,
            vote_user_list: Vec::new(),
        };
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn vote_gif(ctx:Context<VoteGif>, gif_index:String)->ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let vote_user = &mut ctx.accounts.vote_user;
        base_account.gif_list[gif_index.parse::<usize>().unwrap()].num_votes += 1;
        base_account.gif_list[gif_index.parse::<usize>().unwrap()].vote_user_list.push(*vote_user.to_account_info().key);
        Ok(())
    }
}


#[derive(Accounts)]
pub struct StartStuffOff<'info>{
    #[account(init, payer=user, space=9000)]
    pub base_account:Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info>{
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct VoteGif<'info>{
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub vote_user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub num_votes: u64,
    pub vote_user_list: Vec<Pubkey>,
}

#[account]
pub struct BaseAccount{
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}
