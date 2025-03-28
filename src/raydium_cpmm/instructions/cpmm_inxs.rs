use anchor_client::{Client, Cluster};
use anyhow::Result;
use solana_program::pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, system_program, sysvar};

use raydium_cp_swap::accounts as raydium_cp_accounts;
use raydium_cp_swap::instruction as raydium_cp_instructions;
use raydium_cp_swap::{
    states::{AMM_CONFIG_SEED, OBSERVATION_SEED, POOL_LP_MINT_SEED, POOL_SEED, POOL_VAULT_SEED},
    AUTH_SEED,
};
use std::rc::Rc;

use crate::raydium_cpmm::cpmm_builder::ClientConfig;
use crate::raydium_cpmm::cpmm_instructions::{
    swap_base_input_ix_with_program_id, swap_base_output_ix_with_program_id, SwapBaseInputIxArgs,
    SwapBaseInputKeys, SwapBaseOutputIxArgs, SwapBaseOutputKeys,
};

pub const RAYDIUM_CPMM: Pubkey = pubkey!("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C");

pub fn initialize_pool_instr(
    config: &ClientConfig,
    payer: Keypair,
    token_0_mint: Pubkey,
    token_1_mint: Pubkey,
    token_0_program: Pubkey,
    token_1_program: Pubkey,
    user_token_0_account: Pubkey,
    user_token_1_account: Pubkey,
    create_pool_fee: Pubkey,
    init_amount_0: u64,
    init_amount_1: u64,
    open_time: u64,
) -> Result<Vec<Instruction>> {
    let url = Cluster::Custom(config.http_url.clone(), config.ws_url.clone());
    // Client.
    let client = Client::new(url, Rc::new(payer));
    let program = client.program(config.raydium_cp_program)?;

    let amm_config_index = 0u16;
    let (amm_config_key, __bump) = Pubkey::find_program_address(
        &[AMM_CONFIG_SEED.as_bytes(), &amm_config_index.to_be_bytes()],
        &program.id(),
    );

    let (pool_account_key, __bump) = Pubkey::find_program_address(
        &[
            POOL_SEED.as_bytes(),
            amm_config_key.to_bytes().as_ref(),
            token_0_mint.to_bytes().as_ref(),
            token_1_mint.to_bytes().as_ref(),
        ],
        &program.id(),
    );
    let (authority, __bump) = Pubkey::find_program_address(&[AUTH_SEED.as_bytes()], &program.id());
    let (token_0_vault, __bump) = Pubkey::find_program_address(
        &[
            POOL_VAULT_SEED.as_bytes(),
            pool_account_key.to_bytes().as_ref(),
            token_0_mint.to_bytes().as_ref(),
        ],
        &program.id(),
    );
    let (token_1_vault, __bump) = Pubkey::find_program_address(
        &[
            POOL_VAULT_SEED.as_bytes(),
            pool_account_key.to_bytes().as_ref(),
            token_1_mint.to_bytes().as_ref(),
        ],
        &program.id(),
    );
    let (lp_mint_key, __bump) = Pubkey::find_program_address(
        &[
            POOL_LP_MINT_SEED.as_bytes(),
            pool_account_key.to_bytes().as_ref(),
        ],
        &program.id(),
    );
    let (observation_key, __bump) = Pubkey::find_program_address(
        &[
            OBSERVATION_SEED.as_bytes(),
            pool_account_key.to_bytes().as_ref(),
        ],
        &program.id(),
    );

    let instructions = program
        .request()
        .accounts(raydium_cp_accounts::Initialize {
            creator: program.payer(),
            amm_config: amm_config_key,
            authority,
            pool_state: pool_account_key,
            token_0_mint,
            token_1_mint,
            lp_mint: lp_mint_key,
            creator_token_0: user_token_0_account,
            creator_token_1: user_token_1_account,
            creator_lp_token: spl_associated_token_account::get_associated_token_address(
                &program.payer(),
                &lp_mint_key,
            ),
            token_0_vault,
            token_1_vault,
            create_pool_fee,
            observation_state: observation_key,
            token_program: spl_token::id(),
            token_0_program,
            token_1_program,
            associated_token_program: spl_associated_token_account::id(),
            system_program: system_program::id(),
            rent: sysvar::rent::id(),
        })
        .args(raydium_cp_instructions::Initialize {
            init_amount_0,
            init_amount_1,
            open_time,
        })
        .instructions()?;
    Ok(instructions)
}

pub fn swap_base_input_instr(
    payer: Pubkey,
    pool_id: Pubkey,
    amm_config: Pubkey,
    observation_account: Pubkey,
    input_token_account: Pubkey,
    output_token_account: Pubkey,
    input_vault: Pubkey,
    output_vault: Pubkey,
    input_token_mint: Pubkey,
    output_token_mint: Pubkey,
    input_token_program: Pubkey,
    output_token_program: Pubkey,
    amount_in: u64,
    minimum_amount_out: u64,
) -> Result<Vec<Instruction>> {
    let (authority, __bump) = Pubkey::find_program_address(&[AUTH_SEED.as_bytes()], &RAYDIUM_CPMM);

    //print all pubkeys
    println!("Payer: {:?}", payer);
    println!("Authority: {:?}", authority);
    println!("Pool ID: {:?}", pool_id);
    println!("AMM Config: {:?}", amm_config);
    println!("Observation Account: {:?}", observation_account);
    println!("Input Token Account: {:?}", input_token_account);
    println!("Output Token Account: {:?}", output_token_account);
    println!("Input Vault: {:?}", input_vault);
    println!("Output Vault: {:?}", output_vault);
    println!("Input Token Mint: {:?}", input_token_mint);
    println!("Output Token Mint: {:?}", output_token_mint);
    println!("Input Token Program: {:?}", input_token_program);
    println!("Output Token Program: {:?}", output_token_program);

    println!("Amount In: {:?}", amount_in);
    println!("Minimum Amount Out: {:?}", minimum_amount_out);

    let instructions = swap_base_input_ix_with_program_id(
        RAYDIUM_CPMM,
        SwapBaseInputKeys {
            payer,
            authority,
            amm_config,
            pool_state: pool_id,
            input_token_account,
            output_token_account,
            input_vault,
            output_vault,
            input_token_program,
            output_token_program,
            input_token_mint,
            output_token_mint,
            observation_state: observation_account,
        },
        SwapBaseInputIxArgs {
            amount_in,
            minimum_amount_out,
        },
    )?;

    Ok([instructions].to_vec())
}

pub fn swap_base_output_instr(
    payer: Pubkey,
    pool_id: Pubkey,
    amm_config: Pubkey,
    observation_account: Pubkey,
    input_token_account: Pubkey,
    output_token_account: Pubkey,
    input_vault: Pubkey,
    output_vault: Pubkey,
    input_token_mint: Pubkey,
    output_token_mint: Pubkey,
    input_token_program: Pubkey,
    output_token_program: Pubkey,
    max_amount_in: u64,
    amount_out: u64,
) -> Result<Vec<Instruction>> {
    let (authority, __bump) = Pubkey::find_program_address(&[AUTH_SEED.as_bytes()], &RAYDIUM_CPMM);

    // let instructions = program
    //     .request()
    //     .accounts(raydium_cp_accounts::Swap {
    //         payer: program.payer(),
    //         authority,
    //         amm_config,
    //         pool_state: pool_id,
    //         input_token_account,
    //         output_token_account,
    //         input_vault,
    //         output_vault,
    //         input_token_program,
    //         output_token_program,
    //         input_token_mint,
    //         output_token_mint,
    //         observation_state: observation_account,
    //     })
    //     .args(raydium_cp_instructions::SwapBaseOutput {
    //         max_amount_in,
    //         amount_out,
    //     })
    //     .instructions()?;

    let instructions = swap_base_output_ix_with_program_id(
        RAYDIUM_CPMM,
        SwapBaseOutputKeys {
            payer,
            authority,
            amm_config,
            pool_state: pool_id,
            input_token_account,
            output_token_account,
            input_vault,
            output_vault,
            input_token_program,
            output_token_program,
            input_token_mint,
            output_token_mint,
            observation_state: observation_account,
        },
        SwapBaseOutputIxArgs {
            max_amount_in,
            amount_out,
        },
    )?;
    Ok([instructions].to_vec())
}
