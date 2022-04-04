/*
 * test space for running book examples
 */
fn alter_number(number: &mut i8) {
    *number += 1;
}

fn print_number(number: i8) {
    println!("print function scope: {}", number);
}

fn main() {
    let mut one: i8 = 1;
    print_number(one);
    alter_number(&mut one);
    println!("main scope: {}", one);

    // build block instance
    let mut stock: Stock = Stock::new("MonolithAi", 95.0); // needs to be mut
    stock.update_price(128.4);  // update stock price
    println!("Here is the stock: {}", stock.current_price);
    stock.print();
    stock.transfer_stock();
    let stock_two: Stock = Stock::new("RIMES", 150.4).with_stop_loss(55.0); // adding optional
    let stock_three: Stock = Stock::new("BUMPER (former known as ASF)", 120.0)
        .with_take_profit(100.0)
        .with_stop_loss(50.0); // chaining
}

/*
 * python object to rust struct:
 *
 * class Stock:
 *     def __init__(self, name: str, open_price: float, stop_loss=0.0, take_profit: float = 0.0) -> None:
 *         self.name: str = name
 *         self.open_price: float = open_price
 *         self.stop_loss: float = stop_loss
 *         self.take_profit: float = take_profit
 *         self.current_price: float = open_price
 *     def update_price(self, new_price: float) -> None:
 *         self.current_price = new_price
 */

// python to rust
// if stock crosses thresholds (stop_loss, take_profit) forces sale
struct Stock {
    name: String,
    open_price: f32,
    stop_loss: f32,
    take_profit: f32,
    current_price: f32
}

// functions that belong to struct ^ go inside an impl block
// constructor
impl Stock {
    fn new(stock_name: &str, price: f32) -> Stock {
        return Stock{
            name: String::from(stock_name),
            open_price: price,
            stop_loss: 0.0,
            take_profit: 0.0,
            current_price: price
        }
    }

    // optional params from python code
    // take stuct, mutate field and return
    fn with_stop_loss(mut self, value: f32) -> Stock {
        self.stop_loss = value;
        return self
    }

    fn with_take_profit(mut self, value: f32) -> Stock {
        self.take_profit = value;
        return self
    }

    fn update_price(&mut self, value: f32) {
        self.current_price = value;
    }
}

trait CanTransfer {
    fn transfer_stock(&self) -> ();

    fn print(&self) -> () {
        println!("a transfer is happening");
    }
}

impl CanTransfer for Stock {
    fn transfer_stock(&self) -> () {
        println!("the stock {} is being transferred for ${}", self.name, self.current_price);
    }
}

// fn accepts all structs that implement CanTransfer trait
fn process_transfer(stock: impl CanTransfer) -> () {
    stock.print();
    stock.transfer_stock();
}