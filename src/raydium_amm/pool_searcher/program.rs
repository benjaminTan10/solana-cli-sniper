use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, instruction::AccountMeta,
    msg, program::invoke, pubkey::Pubkey,
};

pub mod program {
    solana_program::declare_id!("Fq6aKMBQcNpL41JqSgkx2zoiyL3yFaTTtYfLbZLvM6pV");
}

pub mod raydium {
    solana_program::declare_id!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    msg!("SHITTER");

    let mut account_metas = accounts
        .iter()
        .map(|account| AccountMeta {
            pubkey: *account.key,
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        })
        .collect::<Vec<AccountMeta>>();

    let _account = account_metas.remove(0);

    invoke(
        &solana_program::instruction::Instruction {
            program_id: raydium::ID,
            accounts: account_metas.clone(),
            data: data.to_vec(),
        },
        accounts,
    )?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_process_instruction() {
        let program_id = Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8").unwrap();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert!(process_instruction(&program_id, &accounts, &instruction_data).is_ok());
    }
}
