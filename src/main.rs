mod matching_engine;

use matching_engine::{
    engine::{MatchingEngine, TradingPair},
    order_book::*,
};
use rust_decimal_macros::dec;

fn main() {
    let buy_order_from_alice = Order::new(BidOrAsk::Bid, 5.5);
    let buy_order_from_bob = Order::new(BidOrAsk::Bid, 5.5);
    let mut order_book = OrderBook::new();
    order_book.add_limit_order(dec!(5.5), buy_order_from_alice.clone());
    order_book.add_limit_order(dec!(5.5), buy_order_from_bob.clone());

    let sell_order_from_bob = Order::new(BidOrAsk::Ask, 6.5);
    order_book.add_limit_order(dec!(20), sell_order_from_bob.clone());
    println!("{:#?}", order_book);

    let mut engine = MatchingEngine::new();
    let btc_usd = TradingPair::new("BTC".to_string(), "USD".to_string());
    let eth_usd = TradingPair::new("ETH".to_string(), "USD".to_string());
    engine.add_new_market(btc_usd.clone());
    engine
        .place_limit_order(btc_usd, dec!(10.0), buy_order_from_alice.clone())
        .unwrap();
}
