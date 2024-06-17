v1
use std::io;
use rand::Rng;

struct DigitalCallOption {
    underlying_price: f64,
    strike_price: f64,
    barrier_price: f64,
    implied_volatility: f64, // Assumes constant implied volatility for all maturities and strikes
    time_to_maturity: f64,
}

impl DigitalCallOption {
    fn new() -> DigitalCallOption {
        DigitalCallOption {
            underlying_price: 0.0,
            strike_price: 0.0,
            barrier_price: 0.0,
            implied_volatility: 0.0,
            time_to_maturity: 0.0,
        }
    }

    fn get_inputs(&mut self) {
        println!("Enter the underlying price:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        self.underlying_price = input.trim().parse().unwrap();

        println!("Enter the strike price:");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        self.strike_price = input.trim().parse().unwrap();

        println!("Enter the barrier price:");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        self.barrier_price = input.trim().parse().unwrap();

        println!("Enter the implied volatility (as a decimal):");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        self.implied_volatility = input.trim().parse().unwrap();

        println!("Enter the time to maturity (in years):");
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        self.time_to_maturity = input.trim().parse().unwrap();
    }

    fn price(&self, method: &str) -> f64 {
        match method {
            "Monte Carlo" => self.monte_carlo_price(100000),
            "FEM" => self.fem_price(100),
            _ => panic!("Invalid pricing method"),
        }
    }

    fn monte_carlo_price(&self, num_simulations: usize) -> f64 {
        let mut rng = rand::thread_rng();
        let mut payoff_sum = 0.0;

        for _ in 0..num_simulations {
            let z: f64 = rng.gen_range(-3.0..3.0);
            let stock_price = self.underlying_price * f64::exp((self.implied_volatility * f64::sqrt(self.time_to_maturity) * z) - 0.5 * self.implied_volatility.powi(2) * self.time_to_maturity);

            if stock_price > self.barrier_price {
                payoff_sum += 1.0;
            }
        }

        payoff_sum / num_simulations as f64
    }

    fn fem_price(&self, num_elements: usize) -> f64 {
        let dt = self.time_to_maturity / num_elements as f64;
        let mut prices = vec![0.0; num_elements + 1];
        prices[0] = self.underlying_price;

        for i in 1..=num_elements {
            prices[i] = prices[i - 1] + self.implied_volatility * f64::sqrt(dt) * rand::random::<f64>();
        }

        let mut payoff = 0.0;
        if prices[num_elements] > self.barrier_price {
            payoff = 1.0;
        }

        payoff
    }
}

fn main() {
    let mut option = DigitalCallOption::new();
    option.get_inputs();

    println!("Choose the pricing method (Monte Carlo/FEM):");
    let mut method = String::new();
    io::stdin().read_line(&mut method).unwrap();
    let method = method.trim();

    let price = option.price(method);
    println!("The price of the digital call option is: {}", price);
}