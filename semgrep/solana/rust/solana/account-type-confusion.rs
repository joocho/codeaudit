// ------- Account Types -------- 
pub struct Config {
    pub admin: Pubkey,
    pub fee: u32,
    pub user_count: u32,
}

pub struct User {
    pub user_authority: Pubkey,
    pub balance: u64,
}

// ------- Helper functions --------
fn unpack_config(account: &AccountInfo) -> Result<Config, ProgramError> {
    let mut config: Config = deserialize(&mut account.data.borrow())?;

    return config;
}


// ------- Contract Instructions ---------
fn create_user(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let user = next_account_info(account_iter)?;    
   
    // ...
    // Initialize a User struct, set user_authority 
    // to user and set balance to 0
    // ...
    
    Ok(())
}

fn withdraw_tokens(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let vault = next_account_info(account_iter)?;
    let admin = next_account_info(account_iter)?;
    let config = unpack_config(next_account_info(account_iter)?)?;
    let vault_authority = next_account_info(account_iter)?;
    
    if config.owner != program_id {
        return Err(ProgramError::InvalidConfigAccount);
    }
    
    if config.admin != admin.pubkey() {
        return Err(ProgramError::InvalidAdminAccount);
    }
    
    // ...
    // Transfer funds from vault to admin using vault_authority
    // ...
    
    Ok(())
}