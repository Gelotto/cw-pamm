use cosmwasm_std::{Order, StdResult};
use crate::error::ContractError;

use crate::{
    msg::{MarketBizObject, MarketsResponse},
    state::{
        models::{Market, MarketInfo},
        storage::{MarketId, GLOBAL_STATS, MARKETS, MARKET_INFOS, MARKET_STATS},
    },
};

use super::ReadonlyContext;

pub fn query_markets(ctx: ReadonlyContext) -> Result<MarketsResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;

    // TODO: get this by querying the associated jury contract!
    let winning_pool_id: MarketId = 0;

    let stats = GLOBAL_STATS.load(deps.storage)?;

    let mut markets: Vec<MarketBizObject> = Vec::with_capacity(2);

    for result in MARKETS
        .range(deps.storage, None, None, Order::Ascending)
        .collect::<Vec<StdResult<_>>>()
    {
        let (
            market_id,
            Market {
                reserves,
                offset,
                supply,
                ..
            },
        ) = result?;

        let MarketInfo {
            name,
            description,
            image,
            symbol,
        } = MARKET_INFOS.load(deps.storage, market_id)?;

        let market_stats = MARKET_STATS.load(deps.storage, market_id)?;

        markets.push(MarketBizObject {
            id: market_id,
            winner: winning_pool_id == market_id,
            stats: market_stats,
            symbol,
            name,
            description,
            image,
            reserves,
            offset,
            supply,
        });
    }

    Ok(MarketsResponse { markets, stats })
}
