mod order_book {
    use serde::Deserialize;
    use tree;

    #[derive(Debug)]
    pub struct Book {
        buy_tree: Tree<Order>,
        sell_tree: Tree<Order>,
    }

    #[derive(Deserialize)]
    #[derive(Clone)]
    #[derive(Debug)]
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

    pub fn get() -> Book {
        let client = reqwest::blocking::Client::new();
        let body = client.get(
            "https://api.exchange.coinbase.com/products/ETH-GBP/book?level=2",
        )
        .header(reqwest::header::USER_AGENT, "Rust OrderBook")
        .send()
        .unwrap()
        .json::<OrderBookResponse>()
        .unwrap();

        let mut buy_tree = Tree { tree: Vec::new() };
        buy_tree.populate_from_sorted(None, body.bids.as_slice());
 
        let mut sell_tree = Tree { tree: Vec::new() };
        sell_tree.populate_from_sorted(None, body.asks.as_slice());
        let book = Book{buy_tree, sell_tree};
        book
    }
}
