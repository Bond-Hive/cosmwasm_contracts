use cosmwasm_std::{Response, StdResult};
use cw_storage_plus::Item;
use sylvia::ctx::{InstantiateCtx, QueryCtx, ExecCtx};
use sylvia::{contract, entry_points};

use crate::error::ContractError;
use crate::responses::CountResponse;

pub struct CounterContract {
    pub(crate) count: Item<u32>,
}

#[entry_points]
#[contract]
#[sv::error(ContractError)]
impl CounterContract {
    pub const fn new() -> Self {
        Self {
            count: Item::new("count"),
        }
    }

    #[sv::msg(instantiate)]
    pub fn instantiate(&self, ctx: InstantiateCtx, count: u32) -> StdResult<Response> {
        self.count.save(ctx.deps.storage, &count)?;
        Ok(Response::default())
    }

    #[sv::msg(query)]
    pub fn count(&self, ctx: QueryCtx) -> StdResult<CountResponse> {
        let count = self.count.load(ctx.deps.storage)?;
        Ok(CountResponse { count })
    }

    #[sv::msg(exec)]
    pub fn increment_count(&self, ctx: ExecCtx, ) -> StdResult<Response> {
        self.count
            .update(ctx.deps.storage, |count| -> StdResult<u32> {
                Ok(count + 1)
            })?;
        Ok(Response::default())
    }

    #[sv::msg(exec)]
    pub fn decrement_count(&self, ctx: ExecCtx) -> Result<Response, ContractError> {
        let count = self.count.load(ctx.deps.storage)?;
        if count == 0 {
            return Err(ContractError::CannotDecrementCount);
        }
        self.count.save(ctx.deps.storage, &(count - 1))?;
        Ok(Response::default())
    }
}
