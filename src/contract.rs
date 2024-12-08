use crate::error::ContractError;
use crate::responses::{CountResponse, Cw20AddressResponse};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_json_binary, Addr, Response, StdResult, Uint128, WasmMsg};
use cw20::Cw20ExecuteMsg;
use cw_storage_plus::{Item, Map};
use sylvia::ctx::{ExecCtx, InstantiateCtx, QueryCtx};
use sylvia::{contract, entry_points};

pub struct CounterContract {
    pub(crate) count: Item<u32>,
    pub(crate) admins: Map<Addr, ()>,
    pub(crate) cw_20_address: Item<Addr>,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub count: u32,
    pub cw_20_token_address: String,
}

#[entry_points]
#[contract]
#[sv::error(ContractError)]
#[sv::messages(crate::whitelist as Whitelist)]
impl CounterContract {
    pub const fn new() -> Self {
        Self {
            count: Item::new("count"),
            admins: Map::new("admins"),
            cw_20_address: Item::new("cw_20_address"),
        }
    }

    #[sv::msg(instantiate)]
    pub fn instantiate(
        &self,
        ctx: InstantiateCtx,
        count: u32,
        cw_20_token_address: String,
    ) -> Result<Response, ContractError> {
        // Save initial count
        self.count.save(ctx.deps.storage, &count)?;

        // Validate and save the CW20 token address
        let validated_addr = ctx.deps.api.addr_validate(&cw_20_token_address)?;
        self.cw_20_address.save(ctx.deps.storage, &validated_addr)?;

        Ok(Response::new()
            .add_attribute("method", "instantiate")
            .add_attribute("count", count.to_string())
            .add_attribute("cw_20_token_address", cw_20_token_address))
    }

    #[sv::msg(query)]
    pub fn count(&self, ctx: QueryCtx) -> StdResult<CountResponse> {
        let count = self.count.load(ctx.deps.storage)?;
        Ok(CountResponse { count })
    }

    #[sv::msg(query)]
    pub fn cw_20_address(&self, ctx: QueryCtx) -> StdResult<Cw20AddressResponse> {
        let cw_20_address = self.cw_20_address.load(ctx.deps.storage)?;
        Ok(Cw20AddressResponse { cw_20_address })
    }

    #[sv::msg(exec)]
    pub fn increment_count(&self, ctx: ExecCtx) -> StdResult<Response> {
        self.count
            .update(ctx.deps.storage, |count| -> StdResult<u32> {
                Ok(count + 1)
            })?;

        let cw_20_address = self.cw_20_address.load(ctx.deps.storage)?;
        let mint_msg = Cw20ExecuteMsg::Mint {
            recipient: ctx.info.sender.to_string(),
            amount: Uint128::from(100u128), // Fixed: using Uint128
        };

        let wasm_msg = WasmMsg::Execute {
            contract_addr: cw_20_address.to_string(),
            msg: to_json_binary(&mint_msg)?,
            funds: vec![],
        };

        Ok(Response::new()
            .add_message(wasm_msg)
            .add_attribute("action", "increment_count"))
    }

    #[sv::msg(exec)]
    pub fn decrement_count(&self, ctx: ExecCtx) -> Result<Response, ContractError> {
        let count = self.count.load(ctx.deps.storage)?;
        if count == 0 {
            return Err(ContractError::CannotDecrementCount);
        }
        self.count.save(ctx.deps.storage, &(count - 1))?;
        Ok(Response::new().add_attribute("action", "decrement_count"))
    }
}
