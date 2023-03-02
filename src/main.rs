mod order_book {
    use serde::Deserialize;

    #[derive(Debug)]
    pub struct Limit<'a> {
        left_child: Option<&'a Limit<'a>>,
        right_child: Option<&'a Limit<'a>>,
        price: String,
    }

    #[derive(Debug)]
    pub struct Book<'a> {
        buy_tree: Option<&'a Limit<'a>>,
        sell_tree: Option<&'a Limit<'a>>,
    }

    #[derive(Deserialize)]
    struct Order {
        price: String,
        size: String,
        num_orders: usize
    }

    #[derive(Deserialize)]
    struct OrderBookResponse {
        bids: Vec<Order>,
        asks: Vec<Order>
    }

    pub fn get() -> Book<'static> {
        let client = reqwest::blocking::Client::new();
        let body = client.get(
            "https://api.exchange.coinbase.com/products/ETH-GBP/book?level=2",
        )
        .header(reqwest::header::USER_AGENT, "Rust OrderBook")
        .send()
        .unwrap()
        .json::<OrderBookResponse>()
        .unwrap();

        let buy_tree = populate_tree(body.bids.as_slice());
        let sell_tree = populate_tree(body.asks.as_slice());
        let book = Book{buy_tree, sell_tree};
        book
    }

    fn populate_tree(orders: &[Order]) -> Option<&Limit> {
        if orders.len() == 0 {
            return None;
        }

        let mid_idx = orders.len() / 2;

        let left_child = populate_tree(&orders[..mid_idx]);
        let right_child = populate_tree(&orders[mid_idx+1..]);
        let node = Limit {
            left_child: left_child,
            right_child: right_child,
            price: orders[mid_idx].price.clone(),
        };

        Some(&node)
    }
}

fn main() {
    let book = order_book::get();
    println!("{:?}", book);
}
