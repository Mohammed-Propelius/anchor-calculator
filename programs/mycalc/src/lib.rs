use anchor_lang::prelude::*;

declare_id!("Ax3Dc6AQqrdY6kLgDpeZwrNCihnAK69HvtMBDyYveALQ");

#[program]
pub mod mycalc {
    use super::*;

    pub fn create(ctx:Context<Create>,init_message:String) -> ProgramResult{
        let calculator = &mut ctx.account.calculator
        calculator.gretting = init_message;
        Ok(())
    }

    pub fn add(ctx:Context<Addition>,num1:i64,num2:i64) -> ProgramResult {
        let calculator = &mut ctx.account.calculator
        calculator.result = num1 + num2;
    }
}

#deriver(Accounts)
pub struct create<'info>{
    #[account(init,payer=user,space=264)]
    pub calculator:Account<'info,Calculator>
    #[account(mut)]
    pub user:Signer<'info>
    pub system_program:Program<'info,System>
}

#[account]
pub struct Calculator{
    pub gretting:String,
    pub result:i64,
    pub remainder:i64, 
}




