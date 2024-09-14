use {
    crate::{
        app::config_init::get_config,
        instruction::instruction::{AmmInstruction, RaydiumAmmAccounts, RAYDIUM_AMM_ACCOUNTS_LEN},
        jupiter::interface::{RouteIxData, RouteKeys, ROUTE_IX_ACCOUNTS_LEN},
        pumpfun::instructions::{
            instructions::{calculate_buy_price, get_bonding_curve},
            pumpfun_program::{
                accounts::BondingCurve,
                instructions::{
                    buy_ix_with_program_id, BuyIxArgs, BuyIxData, BuyKeys, BUY_IX_ACCOUNTS_LEN,
                },
                PUMPFUN_PROGRAM,
            },
        },
        raydium_amm::pool_searcher::amm_keys::pool_keys_fetcher,
        router::SniperRoute,
        utils::transaction::send_transaction,
    },
    borsh::BorshDeserialize,
    chrono::{offset::LocalResult, TimeZone, Utc},
    log::{info, warn},
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_program::pubkey::Pubkey,
    solana_sdk::{
        native_token::sol_to_lamports, program_pack::Pack, signature::Keypair, signer::Signer,
        transaction::VersionedTransaction,
    },
    spl_associated_token_account::get_associated_token_address,
    spl_token::state::Mint,
    std::{str::FromStr, sync::Arc},
    yellowstone_grpc_proto::{
        geyser::{CommitmentLevel, SubscribeUpdateTransaction},
        solana::storage::confirmed_block::CompiledInstruction,
    },
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Args {
    /// Service endpoint
    endpoint: String,

    x_token: Option<String>,

    /// Commitment level: processed, confirmed or finalized
    commitment: Option<ArgsCommitment>,

    /// Filter vote transactions
    vote: Option<bool>,

    /// Filter failed transactions
    failed: Option<bool>,

    /// Filter by transaction signature
    signature: Option<String>,

    /// Filter included account in transactions
    account_include: Vec<String>,

    /// Filter excluded account in transactions
    account_exclude: Vec<String>,

    /// Filter required account in transactions
    account_required: Vec<String>,
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
enum ArgsCommitment {
    #[default]
    Processed,
    Confirmed,
    Finalized,
}

impl From<ArgsCommitment> for CommitmentLevel {
    fn from(commitment: ArgsCommitment) -> Self {
        match commitment {
            ArgsCommitment::Processed => CommitmentLevel::Processed,
            ArgsCommitment::Confirmed => CommitmentLevel::Confirmed,
            ArgsCommitment::Finalized => CommitmentLevel::Finalized,
        }
    }
}

pub const RAYDIUM_AMM_V4_PROGRAM_ID: Pubkey =
    solana_sdk::pubkey!("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8");

pub const RAYDIUM_AMM_FEE_COLLECTOR: &str = "7YttLkHDoNj9wyDur5pM1ejNaAvT9X4eqaYcHQqtj2G5";

pub async fn copy_trade_sub(
    tx: SubscribeUpdateTransaction,
    rpc_client: Arc<RpcClient>,
) -> eyre::Result<()> {
    // println!("tx: {:?}", tx);
    let info = tx.clone().transaction.unwrap_or_default();
    let accounts = info
        .transaction
        .clone()
        .unwrap_or_default()
        .message
        .unwrap_or_default()
        .account_keys
        .iter()
        .map(|i| {
            let mut array = [0; 32];
            let bytes = &i[..array.len()];
            array.copy_from_slice(bytes);
            Pubkey::new_from_array(array)
        })
        .collect::<Vec<Pubkey>>();
    let outer_instructions = {
        let transaction = info.transaction.unwrap_or_default();
        let message = transaction.message.unwrap_or_default();
        let instructions = message.instructions.iter();
        instructions.cloned().collect::<Vec<_>>()
    };

    let meta = info.meta.unwrap_or_default();

    let inner_instructions: Vec<CompiledInstruction> = meta
        .clone()
        .inner_instructions
        .iter()
        .flat_map(|inner| {
            inner.instructions.iter().map(|instr| CompiledInstruction {
                program_id_index: instr.program_id_index,
                accounts: instr.accounts.clone(),
                data: instr.data.clone(),
            })
        })
        .collect();

    let signature_base58 = bs58::encode(&info.signature).into_string();

    let mut coin_args: Option<BuyIxData> = None;
    let mut coin_args_amm: Option<AmmInstruction> = None;
    let mut jup_event: Option<RouteIxData> = None;

    let mut trade_route: Option<SniperRoute> = None;
    let mut raydium_accounts: Option<RaydiumAmmAccounts> = None;
    let mut buy_keys: Option<BuyKeys> = None;
    let mut jup_route_keys: Option<RouteKeys> = None;

    for (index, instructions) in outer_instructions.iter().enumerate() {
        match BuyIxData::deserialize(&instructions.data) {
            Ok(decode_new_coin) => {
                coin_args = Some(decode_new_coin);
                trade_route = Some(SniperRoute::PumpFun);

                if instructions.accounts.len() >= BUY_IX_ACCOUNTS_LEN {
                    buy_keys = Some(BuyKeys {
                        global: accounts[instructions.accounts[0] as usize],
                        fee_recipient: accounts[instructions.accounts[1] as usize],
                        mint: accounts[instructions.accounts[2] as usize],
                        bonding_curve: accounts[instructions.accounts[3] as usize],
                        associated_bonding_curve: accounts[instructions.accounts[4] as usize],
                        associated_user: accounts[instructions.accounts[5] as usize],
                        user: accounts[instructions.accounts[6] as usize],
                        system_program: accounts[instructions.accounts[7] as usize],
                        token_program: accounts[instructions.accounts[8] as usize],
                        rent: accounts[instructions.accounts[9] as usize],
                        event_authority: accounts[instructions.accounts[10] as usize],
                        program: accounts[instructions.accounts[11] as usize],
                    });
                }
                break;
            }
            Err(_) => match AmmInstruction::unpack(&instructions.data) {
                Ok(decode_new_coin) => {
                    coin_args_amm = Some(decode_new_coin);
                    trade_route = Some(SniperRoute::RaydiumAMM);

                    if instructions.accounts.len() >= RAYDIUM_AMM_ACCOUNTS_LEN {
                        raydium_accounts = Some(RaydiumAmmAccounts {
                            spl_token: accounts[instructions.accounts[0] as usize],
                            amm_pool: accounts[instructions.accounts[1] as usize],
                            amm_authority: accounts[instructions.accounts[2] as usize],
                            amm_open_orders: accounts[instructions.accounts[3] as usize],
                            amm_target_orders: accounts[instructions.accounts[4] as usize],
                            amm_coin_vault: accounts[instructions.accounts[5] as usize],
                            amm_pc_vault: accounts[instructions.accounts[6] as usize],
                            market_program: Pubkey::from_str(
                                "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX",
                            )?,
                            market: accounts[instructions.accounts[8] as usize],
                            market_bids: accounts[instructions.accounts[9] as usize],
                            market_asks: accounts[instructions.accounts[10] as usize],
                            market_event_queue: accounts[instructions.accounts[11] as usize],
                            market_coin_vault: accounts[instructions.accounts[12] as usize],
                            market_pc_vault: accounts[instructions.accounts[13] as usize],
                            market_vault_signer: accounts[instructions.accounts[14] as usize],
                            source_token_account: accounts[instructions.accounts[15] as usize],
                            destination_token_account: accounts[instructions.accounts[16] as usize],
                            user_source_owner: accounts[instructions.accounts[17] as usize],
                        });
                    }
                    break;
                }
                Err(_) => {
                    let mut data_slice = instructions.data.as_slice();
                    match RouteIxData::deserialize(&mut data_slice) {
                        Ok(decode_new_coin) => {
                            jup_event = Some(decode_new_coin);
                            trade_route = Some(SniperRoute::Jupiter);

                            if instructions.accounts.len() >= ROUTE_IX_ACCOUNTS_LEN {
                                jup_route_keys = Some(RouteKeys {
                                    token_program: accounts[instructions.accounts[0] as usize],
                                    user_transfer_authority: accounts
                                        [instructions.accounts[1] as usize],
                                    user_source_token_account: accounts
                                        [instructions.accounts[2] as usize],
                                    user_destination_token_account: accounts
                                        [instructions.accounts[3] as usize],
                                    destination_token_account: accounts
                                        [instructions.accounts[4] as usize],
                                    destination_mint: accounts[instructions.accounts[5] as usize],
                                    platform_fee_account: accounts
                                        [instructions.accounts[6] as usize],
                                    event_authority: accounts[instructions.accounts[7] as usize],
                                    program: accounts[instructions.accounts[8] as usize],
                                })
                            }
                            break;
                        }
                        Err(_) => {
                            continue;
                        }
                    }
                }
            },
        }
    }

    if trade_route
        .as_ref()
        .map_or(false, |route| *route == SniperRoute::RaydiumAMM)
    {
        let pool_keys = match pool_keys_fetcher(accounts[3], rpc_client.clone()).await {
            Ok(result) => result,
            Err(e) => {
                return Ok(());
            }
        };

        let decoded_transfers: Vec<Option<(Pubkey, Pubkey, u64)>> = inner_instructions
            .iter()
            .map(|ix| decode_transfer(ix, &accounts))
            .collect();

        let user_spl_mint = get_associated_token_address(&accounts[0], &pool_keys.base_mint);

        if decoded_transfers[decoded_transfers.len() - 1].unwrap().1 == user_spl_mint {
            let datetime = match Utc.timestamp_opt(0, 0) {
                LocalResult::Single(datetime) => datetime,
                LocalResult::None => {
                    warn!("Open time is not available");
                    Utc::now()
                }
                LocalResult::Ambiguous(_, _) => {
                    warn!("Open time is out of range");
                    Utc::now()
                }
            };

            let freeze_check = rpc_client.get_account_data(&pool_keys.base_mint).await?;

            let freeze_check = Mint::unpack(&freeze_check).unwrap();

            if freeze_check.freeze_authority.is_some() {
                info!("Freeze Authority set, skipping transaction");
                return Ok(());
            }

            // println!("----------------------------------------");
            // println!("hash: {}", signature_base58);
            // println!("{raydium_accounts:#?}");
            // println!("Freeze Check: {:#?}", freeze_check);

            // let _ = sniper_txn_in_2(pool_keys, 0, None, datetime).await;
        } else {
            return Ok(());
        }
    } else if trade_route
        .as_ref()
        .map_or(false, |route| *route == SniperRoute::PumpFun)
    {
        println!("----------------------------------------");
        println!("hash: {}", signature_base58);
        info!("{:#?}", coin_args);
        let settings_config = get_config().await?;

        if buy_keys.is_some() {
            let wallet = Keypair::from_base58_string(&settings_config.engine.payer_keypair);
            let bonding_curve_pda = get_bonding_curve(buy_keys.unwrap().mint, &PUMPFUN_PROGRAM);
            let bonding_curve_ata =
                get_associated_token_address(&bonding_curve_pda, &buy_keys.unwrap().mint);
            let signer_ata =
                get_associated_token_address(&wallet.pubkey(), &buy_keys.unwrap().mint);

            if let Some(keys) = buy_keys.as_mut() {
                keys.user = wallet.pubkey();
                keys.associated_user = signer_ata;
                keys.associated_bonding_curve = bonding_curve_ata;
            }

            let account_data = rpc_client.get_account_data(&bonding_curve_pda).await?;

            let sliced_data: &mut &[u8] = &mut account_data.as_slice();

            let reserves = BondingCurve::deserialize_reader(sliced_data)?;

            let reserves = (
                reserves.real_token_reserves as u128,
                reserves.virtual_sol_reserves as u128,
                reserves.real_sol_reserves as u128,
            );

            println!("keys: {buy_keys:#?}");

            let price: (u128, (u128, u128, u128)) = calculate_buy_price(
                sol_to_lamports(settings_config.trading.buy_amount) as u128,
                reserves,
            );
            let fee_bps = sol_to_lamports(settings_config.trading.buy_amount) * 1 / 100;

            let args = BuyIxArgs {
                amount: price.0 as u64,
                max_sol_cost: sol_to_lamports(settings_config.trading.buy_amount) + fee_bps,
            };

            let buy_ix = buy_ix_with_program_id(PUMPFUN_PROGRAM, buy_keys.unwrap(), args)?;

            let config = solana_sdk::commitment_config::CommitmentLevel::Finalized;
            let (latest_blockhash, _) = rpc_client
                .get_latest_blockhash_with_commitment(
                    solana_sdk::commitment_config::CommitmentConfig { commitment: config },
                )
                .await?;

            let message = match solana_program::message::v0::Message::try_compile(
                &wallet.pubkey(),
                &[buy_ix],
                &[],
                latest_blockhash,
            ) {
                Ok(x) => x,
                Err(e) => {
                    println!("Error: {:?}", e);
                    return Ok(());
                }
            };

            let transaction = match VersionedTransaction::try_new(
                solana_program::message::VersionedMessage::V0(message),
                &[&wallet],
            ) {
                Ok(x) => x,
                Err(e) => {
                    println!("Error: {:?}", e);
                    return Ok(());
                }
            };

            send_transaction(settings_config, transaction).await?;
        }
    } else if trade_route
        .as_ref()
        .map_or(false, |route| *route == SniperRoute::Jupiter)
    {
        println!("----------------------------------------");
        println!("hash: {}", signature_base58);
        println!("jup: {jup_event:#?}");
        println!("{jup_route_keys:#?}");
    }

    Ok(())
}

fn decode_transfer(
    instruction: &CompiledInstruction,
    accounts: &[Pubkey],
) -> Option<(Pubkey, Pubkey, u64)> {
    if let Ok(lamports) = bincode::deserialize::<u64>(&instruction.data) {
        if instruction.accounts.len() >= 2 {
            let from = accounts[instruction.accounts[0] as usize];
            let to = accounts[instruction.accounts[1] as usize];
            return Some((from, to, lamports));
        }
    }

    None
}
