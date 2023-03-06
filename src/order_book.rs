mod order_book {
    use crate::tree::Tree;
    use serde::Deserialize;
    use std::cmp::Ordering;

    enum OrderType {
        Buy,
        Sell
    }

    #[derive(Clone)]
    pub struct Order {
        price: String,
        size: String,
    }

    #[derive(Clone)]
    pub struct Limit {
        price: String,
        orders: Vec<Order>,
    }

    impl Eq for Limit {}

    impl PartialEq for Limit {
        fn eq(&self, other: &Self) -> bool {
            self.price == other.price
        }
    }

    impl Ord for Limit {
        fn cmp(&self, other: &Self) -> Ordering {
            self.price.cmp(&other.price)
        }
    }

    impl PartialOrd for Limit {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    pub struct Book {
        buy_tree: Tree<Limit>,
        sell_tree: Tree<Limit>,
    }

    #[derive(Deserialize)]
    struct AggregatedOrder {
        price: String,
        size: String,
        num_orders: usize,
    }

    #[derive(Deserialize)]
    struct OrderBookResponse {
        bids: Vec<AggregatedOrder>,
        asks: Vec<AggregatedOrder>,
    }

    fn to_limits<'a>(orders: Vec<AggregatedOrder>) -> Vec<Limit> {
        orders
            .iter()
            .map(|n| {
                let os = vec![
                    Order {
                        price: n.price.clone(),
                        size: n.size.clone()
                    };
                    n.num_orders
                ];
                Limit {
                    price: n.price.clone(),
                    orders: os,
                }
            })
            .collect()
    }

    pub fn get() -> Book {
        let client = reqwest::blocking::Client::new();
        let body = client
            .get("https://api.exchange.coinbase.com/products/ETH-GBP/book?level=2")
            .header(reqwest::header::USER_AGENT, "Rust OrderBook")
            .send()
            .unwrap()
            .json::<OrderBookResponse>()
            .unwrap();

        let buy_limits = to_limits(body.bids);
        let mut buy_tree = Tree::new();
        buy_tree.populate_from_sorted(None, &buy_limits.as_slice());

        let mut sell_tree = Tree::new();
        let sell_limits = to_limits(body.asks);
        sell_tree.populate_from_sorted(None, &sell_limits.as_slice());
        let book = Book {
            buy_tree,
            sell_tree,
        };
        book
    }

    impl Book {
        fn add(&mut self, order_type: OrderType, order: Order) {
            let target = match order_type {
                OrderType::Buy => &mut self.buy_tree,
                OrderType::Sell => &mut self.sell_tree,
            };

            let limit = Limit {
                price: order.price.clone(),
                orders: Vec::new(),
            };
            let l = target.add(limit, 0);
            l.orders.push(order);
        }
    }
}
