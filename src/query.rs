use crate::state::{TokenInfo, TOKEN};
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, StdError};
use crate::msg::{TokenInfoResponse};
use cosmwasm_std::{ QueryResponse};
use crate::error::{ContractError};

pub fn query_get_token_info(deps: Deps) -> Result<QueryResponse, StdError> {
    let token_info = TOKEN.load(deps.storage)?;
    Ok(to_binary(&TokenInfoResponse {
        name: token_info.name.clone(),
        symbol: token_info.symbol.clone(),
        decimal: token_info.decimal,
        description: token_info.description.clone(),
        logo: token_info.logo.clone(),
        creator: token_info.creator.clone(),
    })?)
}
