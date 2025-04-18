use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod signer_authorization_insecure {
    use super::*;

    pub fn before() {
        ()
    }

    /// # Possible fix 1
    ///
    /// ```
    /// if !ctx.accounts.authority.is_signer {
    ///     return Err(ProgramError::MissingRequiredSignature);
    /// }
    /// ```
    ///
    pub fn log_message(ctx: Context<LogMessage>) -> ProgramResult {
        msg!("GM {}", ctx.accounts.authority.key().to_string());
        Ok(())
    }

    pub fn foobar() {
        ()
    }
}
/// # Possible fix 2
///
/// ```
/// authority: Signer<'info>
/// ```
#[derive(Accounts)]
pub struct LogMessage<'info> {
    authority: AccountInfo<'info>,
}
