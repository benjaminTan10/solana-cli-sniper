use {
    crate::{
        app::MevApe,
        pumpfun::instructions::pumpfun_program::instructions::BuyIxData,
        raydium_amm::{
            pool_searcher::amm_keys::pool_keys_fetcher,
            swap::{instructions::AmmInstruction, raydium_amm_sniper::sniper_txn_in_2},
        },
        router::SniperRoute,
    },
    chrono::{offset::LocalResult, TimeZone, Utc},
    futures::{channel::mpsc::SendError, Sink},
    log::{info, warn},
    solana_client::nonblocking::rpc_client::RpcClient,
    solana_program::pubkey::Pubkey,
    solana_sdk::{
        instruction::Instruction, system_instruction::SystemInstruction, system_program,
        system_transaction,
    },
    spl_associated_token_account::get_associated_token_address,
    std::sync::Arc,
    yellowstone_grpc_proto::{
        geyser::SubscribeUpdateTransaction,
        prelude::{CommitmentLevel, SubscribeRequest},
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
    rpc_client: Arc<RpcClient>,
    tx: SubscribeUpdateTransaction,
    mev_ape: Arc<MevApe>,
    mut subscribe_tx: tokio::sync::MutexGuard<
        '_,
        impl Sink<SubscribeRequest, Error = SendError> + std::marker::Unpin,
    >,
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
            let bytes = &i[..array.len()]; // panics if not enough data
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

    let mut coin_args: Option<BuyIxData> = None;
    let mut coin_args_amm: Option<AmmInstruction> = None;

    let mut trade_route = None;
    for instructions in outer_instructions.iter() {
        match BuyIxData::deserialize(&instructions.data) {
            Ok(decode_new_coin) => {
                info!("{:#?}", decode_new_coin);
                coin_args = Some(decode_new_coin);
                trade_route = Some(SniperRoute::PumpFun);
                break;
            }
            Err(_) => match AmmInstruction::unpack(&instructions.data) {
                Ok(decode_new_coin) => {
                    println!("{:#?}", decode_new_coin);
                    coin_args_amm = Some(decode_new_coin);
                    trade_route = Some(SniperRoute::RaydiumAMM);
                    break;
                }
                Err(_) => {
                    continue;
                }
            },
        };
    }

    if trade_route
        .as_ref()
        .map_or(false, |route| *route == SniperRoute::RaydiumAMM)
    {
        info!("Inx Length: {}", inner_instructions.len());

        let pool_keys = match pool_keys_fetcher(accounts[3]).await {
            Ok(result) => result,
            Err(e) => {
                return Ok(());
            }
        };

        println!("Pool Keys: {}", pool_keys.base_mint);

        let decoded_transfers: Vec<Option<(Pubkey, Pubkey, u64)>> = inner_instructions
            .iter()
            .map(|ix| decode_transfer(ix, &accounts))
            .collect();

        let user_spl_mint = get_associated_token_address(&accounts[0], &pool_keys.base_mint);

        println!(
            "{:#?}",
            decoded_transfers[decoded_transfers.len() - 1].unwrap().1
        );
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
            tokio::spawn(async move {
                let _ = sniper_txn_in_2(pool_keys, 0, mev_ape, datetime).await;
            });
        } else {
            return Ok(());
        }
    } else {
    }

    Ok(())
}

fn decode_transfer(
    instruction: &CompiledInstruction,
    accounts: &[Pubkey],
) -> Option<(Pubkey, Pubkey, u64)> {
    // Assuming the system program ID is the first account in the transaction
    // if instruction.program_id_index != 0 {
    //     return None; // Not a system program instruction
    // }

    if let Ok(lamports) = bincode::deserialize::<u64>(&instruction.data) {
        if instruction.accounts.len() >= 2 {
            let from = accounts[instruction.accounts[0] as usize];
            let to = accounts[instruction.accounts[1] as usize];
            return Some((from, to, lamports));
        }
    }

    None
}
