enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    fn new(price: f64) -> Price {
        let scalar = 100_000;
        let integral = price as u64;
        let fractional = (price % 1.0) % scalar as f64;
        let fractional = fractional as u64;

        Price {
            scalar,
            integral,
            fractional,
        }
    }
}

struct Limit {
    price: Price,
    orders: Vec<Order>,
}

struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
    fn new(bid_or_ask: BidOrAsk, size: f64) -> Self {
        Order { bid_or_ask, size }
    }
}

fn main() {
    let price = Price::new(50.5);

    println!("{:?}", price);

    let x = "hi";
    let y: String = "hello".into();
    let z = "hey";

    let l1 = longest(x, &y);
    let l2 = longest(l1, z);
}
