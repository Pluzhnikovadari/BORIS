use codec::Compact;
use sp_core::crypto::Pair;
use sp_core::H256 as Hash;
use sp_runtime::MultiSignature;
use sp_std::prelude::*;

#[cfg(feature = "std")]
use crate::{compose_extrinsic, Api};

use super::xt_primitives::*;

pub const CONTRACTS_MODULE: &str = "Contract";
pub const CONTRACTS_PUT_CODE: &str = "put_code";
pub const CONTRACTS_INSTANTIATE: &str = "instantiate";
pub const CONTRACTS_CALL: &str = "call";

type CallIndex = [u8; 2];

type Gas = u64;
type Data = Vec<u8>;
type Balance = u128;

type GasLimit = Compact<Gas>;
type Endowment = Compact<Balance>;
type Value = Compact<Balance>;
type Destination = GenericAddress;

pub type ContractPutCodeFn = (CallIndex, GasLimit, Data);
pub type ContractInstantiateFn = (CallIndex, Endowment, GasLimit, Hash, Data);
pub type ContractCallFn = (CallIndex, Destination, Value, GasLimit, Data);

pub type ContractPutCodeXt = UncheckedExtrinsicV4<ContractPutCodeFn>;
pub type ContractInstantiateXt = UncheckedExtrinsicV4<ContractInstantiateFn>;
pub type ContractCallXt = UncheckedExtrinsicV4<ContractCallFn>;

#[cfg(feature = "std")]
impl<P> Api<P>
where
    P: Pair,
    MultiSignature: From<P::Signature>,
{
    pub fn contract_put_code(&self, gas_limit: Gas, code: Data) -> ContractPutCodeXt {
        compose_extrinsic!(
            &self,
            CONTRACTS_MODULE,
            CONTRACTS_PUT_CODE,
            Compact(gas_limit),
            code
        )
    }

    pub fn contract_instantiate(
        &self,
        endowment: Balance,
        gas_limit: Gas,
        code_hash: Hash,
        data: Data,
    ) -> ContractInstantiateXt {
        compose_extrinsic!(
            self,
            CONTRACTS_MODULE,
            CONTRACTS_INSTANTIATE,
            Compact(endowment),
            Compact(gas_limit),
            code_hash,
            data
        )
    }

    pub fn contract_call(
        &self,
        dest: GenericAddress,
        value: Balance,
        gas_limit: Gas,
        data: Data,
    ) -> ContractCallXt {
        compose_extrinsic!(
            self,
            CONTRACTS_MODULE,
            CONTRACTS_CALL,
            dest,
            Compact(value),
            Compact(gas_limit),
            data
        )
    }
}
