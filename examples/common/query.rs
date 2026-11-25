use ctp_base::{QueryRow, QueryRows};

pub fn append_query_row(rows: &mut QueryRows, row: QueryRow) -> Result<(), String> {
    match (rows, row) {
        (QueryRows::Orders(rows), QueryRow::Order(row)) => rows.push(*row),
        (QueryRows::Trades(rows), QueryRow::Trade(row)) => rows.push(*row),
        (QueryRows::Positions(rows), QueryRow::InvestorPosition(row)) => rows.push(*row),
        (QueryRows::PositionDetails(rows), QueryRow::InvestorPositionDetail(row)) => {
            rows.push(*row)
        }
        (QueryRows::TradingAccounts(rows), QueryRow::TradingAccount(row)) => rows.push(*row),
        (QueryRows::Instruments(rows), QueryRow::Instrument(row)) => rows.push(*row),
        (QueryRows::Exchanges(rows), QueryRow::Exchange(row)) => rows.push(*row),
        (QueryRows::Products(rows), QueryRow::Product(row)) => rows.push(*row),
        (QueryRows::DepthMarketData(rows), QueryRow::DepthMarketData(row)) => rows.push(*row),
        (QueryRows::Investors(rows), QueryRow::Investor(row)) => rows.push(*row),
        (QueryRows::SettlementInfo(rows), QueryRow::SettlementInfo(row)) => rows.push(*row),
        _ => return Err("查询响应与目标视图类型不符".to_owned()),
    }
    Ok(())
}
