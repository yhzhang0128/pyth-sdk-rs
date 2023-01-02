use pyth_sdk::PriceFeed;
use anchor_lang::prelude::*;
use std::collections::{BTreeMap, BTreeSet};
use pyth_sdk_solana::state::load_price_account;

use crate::ErrorCode;

#[account]
pub struct AdminConfig {
    pub loan_price_feed_id:       Pubkey,
    pub collateral_price_feed_id: Pubkey,
}

#[derive(Clone)]
pub struct PythPriceAccount<'info> {
    account: PriceFeed,
    info: AccountInfo<'info>,
}

impl<'a> PythPriceAccount<'a> {
    fn new(info: AccountInfo<'a>, account: PriceFeed) -> PythPriceAccount<'a> {
        Self { info, account }
    }

    /// Deserializes the given `info` into a `Account`.
    #[inline(never)]
    pub fn try_from(info: &AccountInfo<'a>) -> Result<PythPriceAccount<'a>> {
        let mut data: &[u8] = &info.try_borrow_data()?;
        let account = load_price_account(data)
            .map_err(|_x| error!(ErrorCode::PythError))?;
        // CHECK: using a dummy key for constructing PriceFeed
        let feed = account.to_price_feed(&info.key);
        Ok(PythPriceAccount::new(info.clone(), feed))
    }

    pub fn into_inner(self) -> PriceFeed {
        self.account
    }
}

impl<'info> Accounts<'info>
    for PythPriceAccount<'info>
{
    #[inline(never)]
    fn try_accounts(
        _program_id: &Pubkey,
        accounts: &mut &[AccountInfo<'info>],
        _ix_data: &[u8],
        _bumps: &mut BTreeMap<String, u8>,
        _reallocs: &mut BTreeSet<Pubkey>,
    ) -> Result<Self> {
        // if accounts.is_empty() {
        //     return Err(ErrorCode::AccountNotEnoughKeys.into());
        // }
        let account = &accounts[0];
        *accounts = &accounts[1..];
        PythPriceAccount::try_from(account)
    }
}

impl<'info> AccountsExit<'info>
    for PythPriceAccount<'info>
{
    fn exit(&self, program_id: &Pubkey) -> Result<()> {
        // Only persist if the owner is the current program and the account is not closed.
        // if !crate::common::is_closed(&self.info) {
        //     let info = self.to_account_info();
        //     let mut data = info.try_borrow_mut_data()?;
        //     let dst: &mut [u8] = &mut data;
        //     let mut writer = BpfWriter::new(dst);
        //     self.account.try_serialize(&mut writer)?;
        // }
        Ok(())
    }
}

// impl<'info> AccountsClose<'info>
//     for PythPriceAccount<'info>
// {
//     fn close(&self, sol_destination: AccountInfo<'info>) -> Result<()> {
//         crate::common::close(self.to_account_info(), sol_destination)
//     }
// }

impl<'info> ToAccountMetas
    for PythPriceAccount<'info>
{
    fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<AccountMeta> {
        let is_signer = is_signer.unwrap_or(self.info.is_signer);
        let meta = match self.info.is_writable {
            false => AccountMeta::new_readonly(*self.info.key, is_signer),
            true => AccountMeta::new(*self.info.key, is_signer),
        };
        vec![meta]
    }
}

impl<'info> ToAccountInfos<'info>
    for PythPriceAccount<'info>
{
    fn to_account_infos(&self) -> Vec<AccountInfo<'info>> {
        vec![self.info.clone()]
    }
}

// impl<'info> AsRef<AccountInfo<'info>>
//     for PythPriceAccount<'info>
// {
//     fn as_ref(&self) -> &AccountInfo<'info> {
//         &self.info
//     }
// }

// impl<'info> AsRef<PriceFeed>
//     for PythPriceAccount<'info>
// {
//     fn as_ref(&self) -> &PriceFeed {
//         &self.account
//     }
// }

// impl<'a> Deref for PythPriceAccount<'a> {
//     fn deref(&self) -> &PriceFeed {
//         &(self).account
//     }
// }

// impl<'a> DerefMut for PythPriceAccount<'a> {
//     fn deref_mut(&mut self) -> &mut PriceFeed {
//         #[cfg(feature = "anchor-debug")]
//         if !self.info.is_writable {
//             solana_program::msg!("The given Account is not mutable");
//             panic!();
//         }
//         &mut self.account
//     }
// }

// impl<'info> Key for PythPriceAccount<'info> {
//     fn key(&self) -> Pubkey {
//         *self.info.key
//     }
// }
