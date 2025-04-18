#[inline(never)] // avoid stack frame limit
fn process_flash_loan(
    program_id: &Pubkey,
    liquidity_amount: u64,
    accounts: &[AccountInfo],
) -> ProgramResult {
    if liquidity_amount == 0 {
        msg!("Liquidity amount provided cannot be zero");
        return Err(LendingError::InvalidAmount.into());
    }

    let account_info_iter = &mut accounts.iter();
    let source_liquidity_info = next_account_info(account_info_iter)?;
    let destination_liquidity_info = next_account_info(account_info_iter)?;
    let reserve_info = next_account_info(account_info_iter)?;
    let reserve_liquidity_fee_receiver_info = next_account_info(account_info_iter)?;
    let host_fee_receiver_info = next_account_info(account_info_iter)?;
    let lending_market_info = next_account_info(account_info_iter)?;
    let lending_market_authority_info = next_account_info(account_info_iter)?;
    let token_program_id = next_account_info(account_info_iter)?;
    let flash_loan_receiver_program_id = next_account_info(account_info_iter)?;

    if program_id == flash_loan_receiver_program_id.key {
        msg!("Lending program cannot be used as the flash loan receiver program provided");
        return Err(LendingError::InvalidFlashLoanReceiverProgram.into());
    }

    let lending_market = LendingMarket::unpack(&lending_market_info.data.borrow())?;
    if lending_market_info.owner != program_id {
        return Err(LendingError::InvalidAccountOwner.into());
    }
    if &lending_market.token_program_id != token_program_id.key {
        msg!("Lending market token program does not match the token program provided");
        return Err(LendingError::InvalidTokenProgram.into());
    }

    let authority_signer_seeds = &[
        lending_market_info.key.as_ref(),
        &[lending_market.bump_seed],
    ];
    let lending_market_authority_pubkey =
        Pubkey::create_program_address(authority_signer_seeds, program_id)?;
    if &lending_market_authority_pubkey != lending_market_authority_info.key {
        msg!(
            "Derived lending market authority does not match the lending market authority provided"
        );
        return Err(LendingError::InvalidMarketAuthority.into());
    }

    let mut reserve = Reserve::unpack(&reserve_info.data.borrow())?;
    if reserve_info.owner != program_id {
        msg!("Reserve provided is not owned by the lending program");
        return Err(LendingError::InvalidAccountOwner.into());
    }
    if &reserve.lending_market != lending_market_info.key {
        msg!("Invalid reserve lending market account");
        return Err(LendingError::InvalidAccountInput.into());
    }
    if &reserve.liquidity.supply_pubkey != source_liquidity_info.key {
        msg!("Reserve liquidity supply must be used as the source liquidity provided");
        return Err(LendingError::InvalidAccountInput.into());
    }
    if &reserve.config.fee_receiver != reserve_liquidity_fee_receiver_info.key {
        msg!("Reserve liquidity fee receiver does not match the reserve liquidity fee receiver provided");
        return Err(LendingError::InvalidAccountInput.into());
    }

    // @FIXME: if u64::MAX is flash loaned, fees should be inclusive as with ordinary borrows
    let flash_loan_amount = if liquidity_amount == u64::MAX {
        reserve.liquidity.available_amount
    } else {
        liquidity_amount
    };

    let flash_loan_amount_decimal = Decimal::from(flash_loan_amount);
    let (origination_fee, host_fee) = reserve
        .config
        .fees
        .calculate_flash_loan_fees(flash_loan_amount_decimal)?;

    let balance_before_flash_loan = Account::unpack(&source_liquidity_info.data.borrow())?.amount;
    let expected_balance_after_flash_loan = balance_before_flash_loan
        .checked_add(origination_fee)
        .ok_or(LendingError::MathOverflow)?;
    let returned_amount_required = flash_loan_amount
        .checked_add(origination_fee)
        .ok_or(LendingError::MathOverflow)?;

    let mut flash_loan_instruction_accounts = vec![
        AccountMeta::new(*destination_liquidity_info.key, false),
        AccountMeta::new(*source_liquidity_info.key, false),
        AccountMeta::new_readonly(*token_program_id.key, false),
    ];
    let mut flash_loan_instruction_account_infos = vec![
        destination_liquidity_info.clone(),
        flash_loan_receiver_program_id.clone(),
        source_liquidity_info.clone(),
        token_program_id.clone(),
    ];
    for account_info in account_info_iter {
        flash_loan_instruction_accounts.push(AccountMeta {
            pubkey: *account_info.key,
            is_signer: account_info.is_signer,
            is_writable: account_info.is_writable,
        });
        flash_loan_instruction_account_infos.push(account_info.clone());
    }

    reserve.liquidity.borrow(flash_loan_amount_decimal)?;
    Reserve::pack(reserve, &mut reserve_info.data.borrow_mut())?;

    spl_token_transfer(TokenTransferParams {
        source: source_liquidity_info.clone(),
        destination: destination_liquidity_info.clone(),
        amount: flash_loan_amount,
        authority: lending_market_authority_info.clone(),
        authority_signer_seeds,
        token_program: token_program_id.clone(),
    })?;

    const RECEIVE_FLASH_LOAN_INSTRUCTION_DATA_SIZE: usize = 9;
    // @FIXME: don't use 0 to indicate a flash loan receiver instruction https://git.io/JGzz9
    const RECEIVE_FLASH_LOAN_INSTRUCTION_TAG: u8 = 0u8;

    let mut data = Vec::with_capacity(RECEIVE_FLASH_LOAN_INSTRUCTION_DATA_SIZE);
    data.push(RECEIVE_FLASH_LOAN_INSTRUCTION_TAG);
    data.extend_from_slice(&returned_amount_required.to_le_bytes());

    invoke(
        &Instruction {
            program_id: *flash_loan_receiver_program_id.key,
            accounts: flash_loan_instruction_accounts,
            data,
        },
        &flash_loan_instruction_account_infos[..],
    )?;

    reserve = Reserve::unpack(&reserve_info.data.borrow())?;
    reserve
        .liquidity
        .repay(flash_loan_amount, flash_loan_amount_decimal)?;
    Reserve::pack(reserve, &mut reserve_info.data.borrow_mut())?;

    let actual_balance_after_flash_loan =
        Account::unpack(&source_liquidity_info.data.borrow())?.amount;
    if actual_balance_after_flash_loan < expected_balance_after_flash_loan {
        msg!("Insufficient reserve liquidity after flash loan");
        return Err(LendingError::NotEnoughLiquidityAfterFlashLoan.into());
    }

    let mut owner_fee = origination_fee;
    if host_fee > 0 {
        owner_fee = owner_fee
            .checked_sub(host_fee)
            .ok_or(LendingError::MathOverflow)?;
        spl_token_transfer(TokenTransferParams {
            source: source_liquidity_info.clone(),
            destination: host_fee_receiver_info.clone(),
            amount: host_fee,
            authority: lending_market_authority_info.clone(),
            authority_signer_seeds,
            token_program: token_program_id.clone(),
        })?;
    }

    if owner_fee > 0 {
        spl_token_transfer(TokenTransferParams {
            source: source_liquidity_info.clone(),
            destination: reserve_liquidity_fee_receiver_info.clone(),
            amount: owner_fee,
            authority: lending_market_authority_info.clone(),
            authority_signer_seeds,
            token_program: token_program_id.clone(),
        })?;
    }

    Ok(())
}