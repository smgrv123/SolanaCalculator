use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod sol_cal {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let base_acc=&mut ctx.accounts.base_acc;     
        base_acc.result=0;
        Ok(())
    }

    pub fn add(ctx:Context<Add>,num1:i64,num2:i64)->ProgramResult{
        let base_acc=&mut ctx.accounts.base_acc;
        base_acc.result=num1+num2;
        Ok(())
    }

    pub fn sub(ctx:Context<Sub>,num1:i64,num2:i64)->ProgramResult{
        let base_acc=&mut ctx.accounts.base_acc;
        base_acc.result=num1-num2;
        Ok(())
    }

    pub fn mul(ctx:Context<Mul>,num1:i64,num2:i64)->ProgramResult{
        let base_acc=&mut ctx.accounts.base_acc;
        base_acc.result=num1*num2;
        Ok(())
    }

    pub fn div(ctx:Context<Div>,num1:i64,num2:i64)->ProgramResult{
        let base_acc=&mut ctx.accounts.base_acc;
        if num1>=num2 {
            base_acc.result=num1/num2;
        }
        else{
            base_acc.result=num2/num1;
        }
        Ok(())
    }

    pub fn modulas(ctx:Context<Modulas>,num1:i64,num2:i64)->ProgramResult{
        let base_acc=&mut ctx.accounts.base_acc;
        base_acc.result=num1%num2;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer=user,space=9000)]
    pub base_acc:Account<'info,Calc>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program:Program<'info,System>
}

#[derive(Accounts)]
pub struct Add<'info>{
    #[account(mut)]
    pub base_acc:Account<'info,Calc>
}

#[derive(Accounts)]
pub struct Sub<'info>{
    #[account(mut)]
    pub base_acc:Account<'info,Calc>
}

#[derive(Accounts)]
pub struct Mul<'info>{
    #[account(mut)]
    pub base_acc:Account<'info,Calc>
}

#[derive(Accounts)]
pub struct Div<'info>{
    #[account(mut)]
    pub base_acc:Account<'info,Calc>
}

#[derive(Accounts)]
pub struct Modulas<'info>{
    #[account(mut)]
    pub base_acc:Account<'info,Calc>
}

#[account]
pub struct Calc{
    pub result:i64
}