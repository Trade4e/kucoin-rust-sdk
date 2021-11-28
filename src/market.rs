use crate::client::{BaseUrl, Environment};
use crate::error::APIError;

/*
@todo
Add full aggregated order book function it's requires api  "General" key permission
*/

type Symbol = Option<String>;
type Name = String;
type BaseCurrency = String;
type QuoteCurrency = String;
type FeeCurrency = String;
type MarketSymbol = Option<String>;
type MinOrderSize = String;
type MinQuoteSize = String;
type MaxOrderSize = String;
type MaxQuoteSize = String;
type IncrementOfTheOrderSize = String;
type IncrementOfTheQuoteSize = String;
type IncrementOfThePrice = String;
type ThresholdForPriceProtection = String;
type AvailableForMargin = bool;
type AvailableForTransaction = bool;

type Sequence = String;
type BestAskPrice = String;
type LastTradedSize = String;
type LastTradedPrice = String;
type BestBidSize = String;
type BestBidPrice = String;
type BestAskSize = String;
type TimeStampData = i64;

type ChangeRateIn24Hour = String;
type ChangePriceIn24Hour = String;
type HighestPriceIn24Hour = String;
type LowestPriceIn24Hour = String;
type VolumeIn24HourOnBaseCurrency = String;
type TradedAmountIn24Hour = String;
type AverageTradingPriceIn24Hours = String;
type TakerFee = String;
type MakerFee = String;
type TakerFeeCoefficient = String;
type MakerFeeCoefficient = String;

struct Market {
    base_url : BaseUrl,
    client: reqwest::Client,
}

#[derive(Default, Debug , PartialOrd, PartialEq, Serialize, Deserialize)]
struct Markets(Vec<String>);


#[derive(Default, Debug , PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SymbolList {
    symbol: Symbol,
    name: Name,
    base_currency: BaseCurrency,
    quote_currency: QuoteCurrency,
    fee_currency: FeeCurrency,
    market: MarketSymbol,
    base_min_size: MinOrderSize,
    quote_min_size: MinQuoteSize,
    base_max_size: MaxOrderSize,
    quote_max_size: MaxQuoteSize,
    base_increment: IncrementOfTheOrderSize,
    quote_increment: IncrementOfTheQuoteSize,
    price_increment: IncrementOfThePrice,
    price_limit_rate: ThresholdForPriceProtection,
    is_margin_enabled: AvailableForMargin,
    enable_trading: AvailableForTransaction
}

#[derive(Debug , PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Level1{
    sequence: Sequence,
    best_ask: BestAskPrice,
    size: LastTradedSize,
    price: LastTradedPrice,
    best_bid_size: BestBidSize,
    best_bid: BestBidPrice,
    best_ask_size: BestAskSize,
    time: TimeStampData
}

#[derive(Debug , PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Ticker {
    symbol: Symbol,   // symbol
    symbol_name:Option<Name>, // Name of trading pairs, it would change after renaming
    buy: BestBidPrice,
    sell: BestAskPrice,    // bestBid
    change_rate: ChangeRateIn24Hour,    // 24h change rate
    change_price: ChangePriceIn24Hour, // 24h change price
    high: HighestPriceIn24Hour,    // 24h highest price
    low: LowestPriceIn24Hour, // 24h lowest price
    vol: VolumeIn24HourOnBaseCurrency, // 24h volume，the aggregated trading volume in BTC
    vol_value: TradedAmountIn24Hour,   // 24h total, the trading volume in quote currency of last 24 hours
    last: LastTradedPrice,  // last price
    average_price: AverageTradingPriceIn24Hours,   // 24h average transaction price yesterday
    taker_fee_rate: TakerFee,    // Basic Taker Fee
    maker_fee_rate: MakerFee,    // Basic Maker Fee
    taker_coefficient: TakerFeeCoefficient,    // Taker Fee Coefficient
    maker_coefficient: MakerFeeCoefficient, // Maker Fee Coefficient
    time: Option<TimeStampData>,
}

#[derive(Debug , PartialOrd, PartialEq, Serialize, Deserialize)]
struct Tickers {
    time: TimeStampData,
    ticker: Vec<Ticker>
}
#[derive(Debug , PartialOrd, PartialEq, Serialize, Deserialize)]
struct Bids(Vec<Vec<(String)>>);

#[derive(Debug , PartialOrd, PartialEq, Serialize, Deserialize)]
struct Asks(Vec<Vec<(String)>>);

#[derive(Debug , PartialOrd, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Level2 {
    time: TimeStampData,
    sequence: Sequence,
    bids: Bids,
    asks: Asks
}

#[derive(Debug, PartialOrd, PartialEq, Serialize, Deserialize)]
struct Response<T> {
    code:String,
    data:T
}

impl Market {
    pub fn create(env:Environment) -> Market {
        Market {
            base_url: BaseUrl::create_base_url(env),
            client:reqwest::Client::new()
        }
    }

    async fn get_symbol_list(&self, market:MarketSymbol) -> Result<Response<Vec<SymbolList>>, APIError> {
        let url = match market {
            Some(T) =>  format!("{}/api/v1/symbols?market={}", self.base_url.value(), T),
            None => format!("{}/api/v1/symbols", self.base_url.value())
        };
        let resp = self.client.get(url).send().await?.json().await?;
        Ok(resp)
    }

    async fn get_order_book_level1(&self, sym:Symbol) -> Result<Response<Level1>, APIError>{
        let url = match sym {
            Some(T) =>  format!("{}/api/v1/market/orderbook/level1?symbol={}", self.base_url.value(), T),
            None => format!("{}/api/v1/market/orderbook/level1?symbol=BTC-USDT", self.base_url.value())
        };
        let resp = self.client.get(url).send().await?.json().await?;
        Ok(resp)
    }

    async fn get_all_tickers(&self) -> Result<Response<Tickers>, APIError>{
        let url = format!("{}/api/v1/market/allTickers", self.base_url.value());
        let resp = self.client.get(url).send().await?.json().await?;
        Ok(resp)
    }

    async fn get_24hr_stats(&self, sym:Symbol) -> Result<Response<Ticker>, APIError>{
        let url = match sym {
            Some(T) =>  format!("{}/api/v1/market/stats?symbol={}", self.base_url.value(), T),
            None => format!("{}/api/v1/market/stats?symbol=BTC-USDT", self.base_url.value())
        };
        let resp = self.client.get(url).send().await?.json().await?;
        Ok(resp)
    }

    async fn get_markets(&self) -> Result<Response<Markets>, APIError>{
        let url = format!("{}/api/v1/markets", self.base_url.value());
        let resp = self.client.get(url).send().await?.json().await?;
        Ok(resp)
    }

    async fn get_orderbook_level2_20(&self, sym:Symbol) -> Result<Response<Level2>, APIError>{
        let url = match sym {
            Some(T) =>  format!("{}/api/v1/market/orderbook/level2_20?symbol={}", self.base_url.value(), T),
            None => format!("{}/api/v1/market/orderbook/level2_20?symbol=BTC-USDT", self.base_url.value())
        };
        let resp = self.client.get(url).send().await?.json().await?;
        Ok(resp)
    }

    async fn get_orderbook_level2_100(&self, sym:Symbol) -> Result<Response<Level2>, APIError>{
        let url = match sym {
            Some(T) =>  format!("{}/api/v1/market/orderbook/level2_100?symbol={}", self.base_url.value(), T),
            None => format!("{}/api/v1/market/orderbook/level2_100?symbol=BTC-USDT", self.base_url.value())
        };
        let resp = self.client.get(url).send().await?.json().await?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_orderbook_level2_100() {
        let market = Market::create(Environment::Live);
        let orderbook = market.get_orderbook_level2_100(Symbol::None).await;
        println!("{:#?}", orderbook);
        //assert!(symbols.is_ok());
    }

    #[tokio::test]
    async fn test_get_orderbook_level2_20() {
        let market = Market::create(Environment::Live);
        let orderbook = market.get_orderbook_level2_20(Symbol::None).await;
        println!("{:#?}", orderbook);
        //assert!(symbols.is_ok());
    }

    #[tokio::test]
    async fn test_get_markets() {
        let market = Market::create(Environment::Live);
        let symbols = market.get_markets().await;
        println!("{:#?}", symbols);
        //assert!(symbols.is_ok());
    }

    #[tokio::test]
    async fn test_get_24hr_stats() {
        let market = Market::create(Environment::Live);
        let symbols = market.get_24hr_stats(Symbol::Some("EOS-USDT".to_string())).await;
        println!("{:#?}", symbols);
        //assert!(symbols.is_ok());
    }

    #[tokio::test]
    async fn test_get_all_tickers() {
        let market = Market::create(Environment::Live);
        let symbols = market.get_all_tickers().await;
        println!("{:#?}", symbols);
        //assert!(symbols.is_ok());
    }

    #[tokio::test]
    async fn test_market_get_symbol_list() {
        let market = Market::create(Environment::Live);
        let symbols = market.get_symbol_list(MarketSymbol::None).await;
        println!("{:#?}", symbols);
        //assert!(symbols.is_ok());
    }

    #[tokio::test]
    async fn test_btc_market_get_symbol_list() {
        let market = Market::create(Environment::Live);
        let symbols = market.get_symbol_list(MarketSymbol::Some("BTC".to_string())).await;
        // println!("{:#?}",&symbols);
    }

    #[tokio::test]
    async fn test_get_order_book_level1() {
        let market = Market::create(Environment::Live);
        let symbols = market.get_order_book_level1(Symbol::Some("BTC-USDT".to_string())).await;
        println!("{:#?}",symbols);
    }
}

