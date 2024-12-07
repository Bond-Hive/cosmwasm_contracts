use cosmwasm_std::{Addr, Response, StdResult, SubMsg, WasmMsg, to_json_binary, Uint128};
use cw_storage_plus::{Item, Map};
use sylvia::ctx::{InstantiateCtx, QueryCtx, ExecCtx};
use sylvia::{contract, entry_points};
use cw20::Cw20ExecuteMsg;
use serde::{Serialize, Deserialize};
use cw20::{Cw20Coin, MinterResponse};

use crate::error::ContractError;
use crate::responses::{CountResponse, Cw20AddressResponse};

pub struct CounterContract {
    pub(crate) count: Item<u32>,
    pub(crate) admins: Map<Addr, ()>,
    pub(crate) cw20_address: Item<Addr>, // Store the address directly
}

#[derive(Serialize, Deserialize)]
pub struct TokenInstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Vec<Cw20Coin>,
    pub mint: Option<MinterResponse>,
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
            cw20_address: Item::new("cw20_address"),
        }
    }

    #[sv::msg(instantiate)]
    pub fn instantiate(&self, ctx: InstantiateCtx, count: u32) -> StdResult<Response> {
        self.count.save(ctx.deps.storage, &count)?;
        Ok(Response::new().add_attribute("method", "instantiate"))
    }

    #[sv::msg(query)]
    pub fn count(&self, ctx: QueryCtx) -> StdResult<CountResponse> {
        let count = self.count.load(ctx.deps.storage)?;
        Ok(CountResponse { count })
    }

    #[sv::msg(query)]
    pub fn cw20_address(&self, ctx: QueryCtx) -> StdResult<Cw20AddressResponse> {
        let cw20_address = self.cw20_address.load(ctx.deps.storage)?;
        Ok(Cw20AddressResponse {
            cw20_address,
        })
    }

    #[sv::msg(exec)]
    pub fn deploy_cw20(
        &self,
        ctx: ExecCtx,
        token_name: String,
        token_symbol: String,
    ) -> StdResult<Response> {
        let instantiate_msg = TokenInstantiateMsg {
            name: token_name,
            symbol: token_symbol,
            decimals: 6,
            initial_balances: vec![],
            mint: Some(MinterResponse {
                minter: ctx.info.sender.to_string(),
                cap: None,
            }),
        };

        let wasm_msg = WasmMsg::Instantiate {
            admin: Some(ctx.info.sender.to_string()),
            code_id: 1, // Replace with actual CW20 code ID
            msg: to_json_binary(&instantiate_msg)?,
            funds: vec![],
            label: "CW20 Token Deployment".to_string(),
        };

        let sub_msg = SubMsg::new(wasm_msg);

        // Save address later in the reply handler
        Ok(Response::new()
            .add_submessage(sub_msg)
            .add_attribute("action", "deploy_cw20"))
    }

    #[sv::msg(exec)]
    pub fn increment_count(&self, ctx: ExecCtx) -> StdResult<Response> {
        self.count.update(ctx.deps.storage, |count| -> StdResult<u32> {
            Ok(count + 1)
        })?;

        let cw20_address = self.cw20_address.load(ctx.deps.storage)?;
        let mint_msg = Cw20ExecuteMsg::Mint {
            recipient: ctx.info.sender.to_string(),
            amount: Uint128::from(100u128), // Fixed: using Uint128
        };

        let wasm_msg = WasmMsg::Execute {
            contract_addr: cw20_address.to_string(),
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
