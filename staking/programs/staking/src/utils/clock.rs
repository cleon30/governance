use anchor_lang::{prelude::*, solana_program::clock::UnixTimestamp};
use crate::{error::ErrorCode, state::global_config::{GlobalConfig}};

/// Computes Pyth clock.
/// Right now it's just the current Unix timestamp divided by the epoch length
pub fn get_current_epoch(global_config : &GlobalConfig) -> Result<u64>{
    let now_ts : u64 = get_current_time(global_config) as u64;
    return now_ts.checked_div(global_config.epoch_duration).ok_or(error!(ErrorCode::ZeroEpochDuration));
}

// As an extra form of defense to make sure we're not using the mock clock
// in devnet or mainnet, we'd like to have an assert(localnet). There's not 
// an easy way to do that, but something that gets close is checking that the
// number of slots that have passed is much smaller than anything possible on
// mainnet or devnet. We set the threshold at 10 million slots, which is more 
// than a month. mainnet, devnet, and testnet are all > 140 million right now.
#[cfg(feature = "mock-clock")]
const MAX_LOCALNET_VALIDATOR_RUNTIME_SLOTS : u64 = 10_000_000;

#[cfg(feature = "mock-clock")]
pub fn get_current_time(global_config : &GlobalConfig) -> UnixTimestamp {

    assert!(Clock::get().unwrap().slot < MAX_LOCALNET_VALIDATOR_RUNTIME_SLOTS);
    global_config.mock_clock_time
}
#[cfg(not(feature = "mock-clock"))]
    pub fn get_current_time(_global_config : &GlobalConfig) -> UnixTimestamp {
    Clock::get().unwrap().unix_timestamp
} 