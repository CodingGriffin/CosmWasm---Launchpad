use crate::state::{TOKEN};
use cosmwasm_std::{to_binary, Deps, StdError};
use crate::msg::{TokenInfoResponse};
use cosmwasm_std::{ QueryResponse};

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
