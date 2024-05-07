use std::{str::FromStr, sync::Arc};

use serde::{Deserialize, Serialize};
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
};
use solana_program::pubkey::Pubkey;

use crate::rpc::HTTP_CLIENT;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct LIQUIDITY_STATE_LAYOUT_V4 {
    pub status: u64,
    pub nonce: u64,
    pub maxOrder: u64,
    pub depth: u64,
    pub baseDecimal: u64,
    pub quoteDecimal: u64,
    pub state: u64,
    pub resetFlag: u64,
    pub minSize: u64,
    pub volMaxCutRatio: u64,
    pub amountWaveRatio: u64,
    pub baseLotSize: u64,
    pub quoteLotSize: u64,
    pub minPriceMultiplier: u64,
    pub maxPriceMultiplier: u64,
    pub systemDecimalValue: u64,
    pub minSeparateNumerator: u64,
    pub minSeparateDenominator: u64,
    pub tradeFeeNumerator: u64,
    pub tradeFeeDenominator: u64,
    pub pnlNumerator: u64,
    pub pnlDenominator: u64,
    pub swapFeeNumerator: u64,
    pub swapFeeDenominator: u64,
    pub baseNeedTakePnl: u64,
    pub quoteNeedTakePnl: u64,
    pub quoteTotalPnl: u64,
    pub baseTotalPnl: u64,
    pub poolOpenTime: u64,
    pub punishPcAmount: u64,
    pub punishCoinAmount: u64,
    pub orderbookToInitTime: u64,
    // u128('poolTotalDepositPc'),
    // u128('poolTotalDepositCoin'),
    pub swapBaseInAmount: u128,
    pub swapQuoteOutAmount: u128,
    pub swapBase2QuoteFee: u64,
    pub swapQuoteInAmount: u128,
    pub swapBaseOutAmount: u128,
    pub swapQuote2BaseFee: u64,
    // amm vault
    pub baseVault: Pubkey,
    pub quoteVault: Pubkey,
    // mint
    pub baseMint: Pubkey,
    pub quoteMint: Pubkey,
    pub lpMint: Pubkey,
    // market
    pub openOrders: Pubkey,
    pub marketId: Pubkey,
    pub marketProgramId: Pubkey,
    pub targetOrders: Pubkey,
    pub withdrawQueue: Pubkey,
    pub lpVault: Pubkey,
    pub owner: Pubkey,
    // true circulating supply without lock up
    pub lpReserve: u64,
    pub padding: [u64; 3],
}

impl LIQUIDITY_STATE_LAYOUT_V4 {
    pub fn decode(input: &mut &[u8]) -> eyre::Result<Self> {
        let mut s = Self::default();
        s.status = Self::unpack_u64(input)?;
        s.nonce = Self::unpack_u64(input)?;
        s.maxOrder = Self::unpack_u64(input)?;
        s.depth = Self::unpack_u64(input)?;
        s.baseDecimal = Self::unpack_u64(input)?;
        s.quoteDecimal = Self::unpack_u64(input)?;
        s.state = Self::unpack_u64(input)?;
        s.resetFlag = Self::unpack_u64(input)?;
        s.minSize = Self::unpack_u64(input)?;
        s.volMaxCutRatio = Self::unpack_u64(input)?;
        s.amountWaveRatio = Self::unpack_u64(input)?;
        s.baseLotSize = Self::unpack_u64(input)?;
        s.quoteLotSize = Self::unpack_u64(input)?;
        s.minPriceMultiplier = Self::unpack_u64(input)?;
        s.maxPriceMultiplier = Self::unpack_u64(input)?;
        s.systemDecimalValue = Self::unpack_u64(input)?;
        s.minSeparateNumerator = Self::unpack_u64(input)?;
        s.minSeparateDenominator = Self::unpack_u64(input)?;
        s.tradeFeeNumerator = Self::unpack_u64(input)?;
        s.tradeFeeDenominator = Self::unpack_u64(input)?;
        s.pnlNumerator = Self::unpack_u64(input)?;
        s.pnlDenominator = Self::unpack_u64(input)?;
        s.swapFeeNumerator = Self::unpack_u64(input)?;
        s.swapFeeDenominator = Self::unpack_u64(input)?;
        s.baseNeedTakePnl = Self::unpack_u64(input)?;
        s.quoteNeedTakePnl = Self::unpack_u64(input)?;
        s.quoteTotalPnl = Self::unpack_u64(input)?;
        s.baseTotalPnl = Self::unpack_u64(input)?;
        s.poolOpenTime = Self::unpack_u64(input)?;
        s.punishPcAmount = Self::unpack_u64(input)?;
        s.punishCoinAmount = Self::unpack_u64(input)?;
        s.orderbookToInitTime = Self::unpack_u64(input)?;
        // u128('poolTotalDepositPc'),
        // u128('poolTotalDepositCoin'),
        s.swapBaseInAmount = Self::unpack_u128(input)?;
        s.swapQuoteOutAmount = Self::unpack_u128(input)?;
        s.swapBase2QuoteFee = Self::unpack_u64(input)?;
        s.swapQuoteInAmount = Self::unpack_u128(input)?;
        s.swapBaseOutAmount = Self::unpack_u128(input)?;
        s.swapQuote2BaseFee = Self::unpack_u64(input)?;
        // amm vault

        s.baseVault = Self::unpack_pubkey(input)?;
        s.quoteVault = Self::unpack_pubkey(input)?;
        // mint
        s.baseMint = Self::unpack_pubkey(input)?;
        s.quoteMint = Self::unpack_pubkey(input)?;
        s.lpMint = Self::unpack_pubkey(input)?;
        // market
        s.openOrders = Self::unpack_pubkey(input)?;
        s.marketId = Self::unpack_pubkey(input)?;
        s.marketProgramId = Self::unpack_pubkey(input)?;
        s.targetOrders = Self::unpack_pubkey(input)?;
        s.withdrawQueue = Self::unpack_pubkey(input)?;
        s.lpVault = Self::unpack_pubkey(input)?;
        s.owner = Self::unpack_pubkey(input)?;
        // true circulating supply without lock up
        s.lpReserve = Self::unpack_u64(input)?;
        s.padding = [
            Self::unpack_u64(input)?,
            Self::unpack_u64(input)?,
            Self::unpack_u64(input)?,
        ];
        Ok(s)
    }
    fn unpack_u64(input: &mut &[u8]) -> eyre::Result<u64> {
        use std::io::Read;

        let mut buf = [0u8; 8];
        input.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }

    fn unpack_u128(input: &mut &[u8]) -> eyre::Result<u128> {
        use std::io::Read;

        let mut buf = [0u8; 16];
        input.read_exact(&mut buf)?;
        Ok(u128::from_le_bytes(buf))
    }
    fn unpack_pubkey(input: &mut &[u8]) -> eyre::Result<Pubkey> {
        use std::io::Read;

        let mut buf = [0u8; 32];
        input.read_exact(&mut buf)?;
        Ok(Pubkey::new_from_array(buf))
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct MARKET_STATE_LAYOUT_V3 {
    pub blob1: [u8; 5],
    pub blob2: [u8; 8],
    pub ownAddress: Pubkey,
    pub vaultSignerNonce: u64,
    pub baseMint: Pubkey,
    pub quoteMint: Pubkey,
    pub baseVault: Pubkey,
    pub baseDepositsTotal: u64,
    pub baseFeesAccrued: u64,
    pub quoteVault: Pubkey,
    pub quoteDepositsTotal: u64,
    pub quoteFeesAccrued: u64,
    pub quoteDustThreshold: u64,
    pub requestQueue: Pubkey,
    pub eventQueue: Pubkey,
    pub bids: Pubkey,
    pub asks: Pubkey,
    pub baseLotSize: u64,
    pub quoteLotSize: u64,
    pub feeRateBps: u64,
    pub referrerRebatesAccrued: u64,
    pub blob3: [u8; 7],
}

impl MARKET_STATE_LAYOUT_V3 {
    pub fn decode(input: &mut &[u8]) -> eyre::Result<Self> {
        let mut s = Self::default();
        s.blob1 = Self::unpack_u8_5(input)?;
        s.blob2 = Self::unpack_u8_8(input)?;
        s.ownAddress = Self::unpack_pubkey(input)?;
        s.vaultSignerNonce = Self::unpack_u64(input)?;
        s.baseMint = Self::unpack_pubkey(input)?;
        s.quoteMint = Self::unpack_pubkey(input)?;
        s.baseVault = Self::unpack_pubkey(input)?;
        s.baseDepositsTotal = Self::unpack_u64(input)?;
        s.baseFeesAccrued = Self::unpack_u64(input)?;
        s.quoteVault = Self::unpack_pubkey(input)?;
        s.quoteDepositsTotal = Self::unpack_u64(input)?;
        s.quoteFeesAccrued = Self::unpack_u64(input)?;
        s.quoteDustThreshold = Self::unpack_u64(input)?;
        s.requestQueue = Self::unpack_pubkey(input)?;
        s.eventQueue = Self::unpack_pubkey(input)?;
        s.bids = Self::unpack_pubkey(input)?;
        s.asks = Self::unpack_pubkey(input)?;
        s.baseLotSize = Self::unpack_u64(input)?;
        s.quoteLotSize = Self::unpack_u64(input)?;
        s.feeRateBps = Self::unpack_u64(input)?;
        s.referrerRebatesAccrued = Self::unpack_u64(input)?;
        s.blob3 = Self::unpack_u8_7(input)?;
        Ok(s)
    }
    fn unpack_u8_5(input: &mut &[u8]) -> eyre::Result<[u8; 5]> {
        use std::io::Read;

        let mut buf = [0u8; 5];
        input.read_exact(&mut buf)?;
        Ok(buf)
    }
    fn unpack_u8_8(input: &mut &[u8]) -> eyre::Result<[u8; 8]> {
        use std::io::Read;

        let mut buf = [0u8; 8];
        input.read_exact(&mut buf)?;
        Ok(buf)
    }
    fn unpack_u8_7(input: &mut &[u8]) -> eyre::Result<[u8; 7]> {
        use std::io::Read;

        let mut buf = [0u8; 7];
        input.read_exact(&mut buf)?;
        Ok(buf)
    }
    fn unpack_u64(input: &mut &[u8]) -> eyre::Result<u64> {
        use std::io::Read;

        let mut buf = [0u8; 8];
        input.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
    fn unpack_pubkey(input: &mut &[u8]) -> eyre::Result<Pubkey> {
        use std::io::Read;

        let mut buf = [0u8; 32];
        input.read_exact(&mut buf)?;
        Ok(Pubkey::new_from_array(buf))
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct SPL_MINT_LAYOUT {
    pub mintAuthorityOption: u32,
    pub mintAuthority: Pubkey,
    pub supply: u64,
    pub decimals: u8,
    pub isInitialized: u8,
    pub freezeAuthorityOption: u32,
    pub freezeAuthority: Pubkey,
}

impl SPL_MINT_LAYOUT {
    pub fn decode(input: &mut &[u8]) -> eyre::Result<Self> {
        let mut s = Self::default();
        s.mintAuthorityOption = Self::unpack_u32(input)?;
        s.mintAuthority = Self::unpack_pubkey(input)?;
        s.supply = Self::unpack_u64(input)?;
        s.decimals = Self::unpack_u8(input)?;
        s.isInitialized = Self::unpack_u8(input)?;
        s.freezeAuthorityOption = Self::unpack_u32(input)?;
        s.freezeAuthority = Self::unpack_pubkey(input)?;
        Ok(s)
    }
    fn unpack_u32(input: &mut &[u8]) -> eyre::Result<u32> {
        use std::io::Read;

        let mut buf = [0u8; 4];
        input.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
    fn unpack_u64(input: &mut &[u8]) -> eyre::Result<u64> {
        use std::io::Read;

        let mut buf = [0u8; 8];
        let _ = match input.read_exact(&mut buf) {
            Ok(_) => Ok(u64::from_le_bytes(buf)),
            Err(e) => Err(eyre::eyre!("Error: {}", e)),
        };
        Ok(u64::from_le_bytes(buf))
    }
    fn unpack_u8(input: &mut &[u8]) -> eyre::Result<u8> {
        use std::io::Read;

        let mut buf = [0u8; 1];
        input.read_exact(&mut buf)?;
        Ok(u8::from_le_bytes(buf))
    }
    fn unpack_pubkey(input: &mut &[u8]) -> eyre::Result<Pubkey> {
        use std::io::Read;

        let mut buf = [0u8; 32];
        input.read_exact(&mut buf)?;
        Ok(Pubkey::new_from_array(buf))
    }
}

pub async fn market_authority(rpc_client: &Arc<RpcClient>, address: Pubkey) -> Pubkey {
    let accounts = rpc_client.get_token_account(&address).await.unwrap();

    let mut serumsigner = Pubkey::default();
    if let Some(account) = accounts {
        serumsigner = Pubkey::from_str(&account.owner).unwrap();
    }

    serumsigner
}

pub async fn program_address(program_id: &Pubkey) -> eyre::Result<Pubkey> {
    let buffer = vec![97, 109, 109, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121];
    let seeds = &[&buffer[..]];

    let (key, bump_seed) = Pubkey::find_program_address(seeds, program_id);
    Ok(key)
}

pub async fn get_associated_authority(program_id: Pubkey) -> eyre::Result<String> {
    let rpc_client = {
        let http_client = HTTP_CLIENT.lock().unwrap();
        http_client.get("http_client").unwrap().clone()
    };

    // Convert the byte array to a Vec<u8>
    let amm_authority_bytes: Vec<u8> =
        vec![97, 109, 109, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121];

    let config = RpcProgramAccountsConfig {
        filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new(
            0,
            MemcmpEncodedBytes::Bytes(amm_authority_bytes.clone()),
        ))]),
        account_config: RpcAccountInfoConfig {
            encoding: Some(UiAccountEncoding::Base64Zstd),
            ..RpcAccountInfoConfig::default()
        },
        ..RpcProgramAccountsConfig::default()
    };

    let accounts = rpc_client
        .get_program_accounts_with_config(&program_id, config)
        .await?;

    let serumsigner = String::from_utf8_lossy(&amm_authority_bytes).to_string();

    Ok(serumsigner)
}
