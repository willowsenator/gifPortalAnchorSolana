use anchor_lang::prelude::*;

declare_id!("FzwXxcH2FVRaqXtXx41N8vxpKPoQY3wRPnJJ7TMjP3da");

fn check_in_list(vote_user_address:Pubkey, vote_user_list:Vec<Pubkey>)->bool{
    for item in vote_user_list.iter(){
        if *item == vote_user_address{
            return true;
        }
    }

    return false;
}

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
            donate_amount: 0
        };
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn vote_gif(ctx:Context<VoteGif>, gif_index:String)->ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let vote_user = &mut ctx.accounts.vote_user;
        if base_account.gif_list[gif_index.parse::<usize>().unwrap()].user_address != *vote_user.to_account_info().key{
            if !check_in_list(*vote_user.to_account_info().key, (*base_account.gif_list[gif_index.parse::<usize>().unwrap()].vote_user_list).to_vec()){
                base_account.gif_list[gif_index.parse::<usize>().unwrap()].num_votes += 1;
                base_account.gif_list[gif_index.parse::<usize>().unwrap()].vote_user_list.push(*vote_user.to_account_info().key);
                Ok(())
            }
            else {
                return Err(ProgramError::Custom(6001));
            }
        }
        else{
            return Err(ProgramError::Custom(6000));
        }
    }

    pub fn donate_to_gif_owner(ctx:Context<DonateToGifOwner>, gif_index:String, amount: u64)->ProgramResult {
        let from = &mut ctx.accounts.from;
        let base_account = &mut ctx.accounts.base_account;


        if base_account.gif_list[gif_index.parse::<usize>().unwrap()].user_address != *from.to_account_info().key {
                    let ix = anchor_lang::solana_program::system_instruction::transfer(
                        &ctx.accounts.from.key(),
                        &ctx.accounts.to.key(),
                        amount,
                    );

                    anchor_lang::solana_program::program::invoke(
                        &ix,
                        &[
                            ctx.accounts.from.to_account_info(),
                            ctx.accounts.to.to_account_info(),
                        ],
                    ).expect("Error to donate");
                    base_account.gif_list[gif_index.parse::<usize>().unwrap()].donate_amount += amount;
                    Ok(())
        }else{
            return Err(ProgramError::Custom(6002));
        }
    }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info>{
    #[account(init, payer=user, space=9000)]
    base_account:Account<'info, BaseAccount>,
    #[account(mut)]
    user: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info>{
    #[account(mut)]
    base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    user: Signer<'info>,
}

#[derive(Accounts)]
pub struct VoteGif<'info>{
    #[account(mut)]
    base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    vote_user: Signer<'info>,
}

#[derive(Accounts)]
pub struct DonateToGifOwner<'info> {
    #[account(mut)]
    from: Signer<'info>,
    /// CHECK:
    to: AccountInfo<'info>,
    #[account(mut)]
    base_account: Account<'info, BaseAccount>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    gif_link: String,
    user_address: Pubkey,
    num_votes: u64,
    vote_user_list: Vec<Pubkey>,
    donate_amount: u64,
}

#[account]
pub struct BaseAccount{
    total_gifs: u64,
    gif_list: Vec<ItemStruct>,
}

#[error_code]
pub enum GifErrors {
    #[msg("You cannot vote with the same gif owner account")]
    VoteAccountNotGifOwnerAccount,
    #[msg("You have already voted")]
    AlreadyVoted,
    #[msg("You cannot donate to yourself")]
    DonateToSameAddress,
}
