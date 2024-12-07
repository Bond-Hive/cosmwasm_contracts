use cosmwasm_std::{Addr, Response, StdError};
use sylvia::interface;
use sylvia::ctx::{ExecCtx, QueryCtx};

use crate::contract::CounterContract;
use crate::error::ContractError;
use crate::responses::AdminsResponse;

#[interface]
pub trait Whitelist {
    type Error: From<StdError>;

    #[sv::msg(exec)]
    fn add_admin(&self, ctx: ExecCtx, address: String) -> Result<Response, Self::Error>;

    #[sv::msg(exec)]
    fn remove_admin(&self, ctx: ExecCtx, address: String) -> Result<Response, Self::Error>;

    #[sv::msg(query)]
    fn admins(&self, ctx: QueryCtx) -> Result<AdminsResponse, Self::Error>;
}

impl Whitelist for CounterContract {
    type Error = ContractError;

    fn add_admin(&self, ctx: ExecCtx, admin: String) -> Result<Response, Self::Error> {
        let deps = ctx.deps;
        let admin = deps.api.addr_validate(&admin)?;
        self.admins.save(deps.storage, admin, &())?;

        Ok(Response::default())
    }

    fn remove_admin(&self, ctx: ExecCtx, admin: String) -> Result<Response, Self::Error> {
        let deps = ctx.deps;
        let admin = deps.api.addr_validate(&admin)?;
        self.admins.remove(deps.storage, admin);

        Ok(Response::default())
    }

    fn admins(&self, ctx: QueryCtx) -> Result<AdminsResponse, Self::Error> {
        let admins: Vec<Addr> = self
            .admins
            .keys(ctx.deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .collect::<Result<_, _>>()?;

        Ok(AdminsResponse { admins })
    }
}
