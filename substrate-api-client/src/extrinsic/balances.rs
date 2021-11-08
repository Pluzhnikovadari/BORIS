use codec::Compact;

use super::xt_primitives::*;
#[cfg(feature = "std")]
use crate::{compose_extrinsic, Api};
use sp_core::crypto::Pair;
use sp_runtime::MultiSignature;

pub const BALANCES_MODULE: &str = "Balances";
pub const BALANCES_TRANSFER: &str = "transfer";
pub const BALANCES_SET_BALANCE: &str = "set_balance";

pub type CallIndex = [u8; 2];
pub type Balance = u128;

pub type BalanceTransferFn = (CallIndex, GenericAddress, Compact<Balance>);
pub type BalanceSetBalanceFn = (
    CallIndex,
    GenericAddress,
    Compact<Balance>,
    Compact<Balance>,
);

pub type BalanceTransferXt = UncheckedExtrinsicV4<BalanceTransferFn>;
pub type BalanceSetBalanceXt = UncheckedExtrinsicV4<BalanceSetBalanceFn>;

#[cfg(feature = "std")]
impl<P> Api<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    pub fn balance_transfer(&self, to: GenericAddress, amount: Balance) -> BalanceTransferXt {
        compose_extrinsic!(
            self,
            BALANCES_MODULE,
            BALANCES_TRANSFER,
            to,
            Compact(amount)
        )
    }

    pub fn balance_set_balance(
        &self,
        who: GenericAddress,
        free_balance: Balance,
        reserved_balance: Balance,
    ) -> BalanceSetBalanceXt {
        compose_extrinsic!(
            self,
            BALANCES_MODULE,
            BALANCES_SET_BALANCE,
            who,
            Compact(free_balance),
            Compact(reserved_balance)
        )
    }
}
