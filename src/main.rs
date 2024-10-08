use std::io;
use rand::Rng;
use statrs::distribution::{Normal, Continuous};

struct DigitalCallOption {
    underlying_price: f64,
    strike_price: f64,
    barrier_price: f64,
    implied_volatility: f64, 
    time_to_maturity: f64,
    risk_free_rate: f64,  // New field for BSM method
}

impl DigitalCallOption {
    fn new() -> DigitalCallOption {
        DigitalCallOption {
            underlying_price: 0.0,
            strike_price: 0.0,
            barrier_price: 0.0,
            implied_volatility: 0.0,
            time_to_maturity: 0.0,
            risk_free_rate: 0.0,  // Initialize risk-free rate
        }
    }

    fn get_inputs(&mut self) {
        println!("Enter the underlying price:");
        self.underlying_price = read_f64();
        println!("Enter the strike price:");
        self.strike_price = read_f64();
        println!("Enter the barrier price:");
        self.barrier_price = read_f64();
        println!("Enter the implied volatility (as a decimal):");
        self.implied_volatility = read_f64();
        println!("Enter the time to maturity (in years):");
        self.time_to_maturity = read_f64();
        println!("Enter the risk-free rate (as a decimal):");
        self.risk_free_rate = read_f64();
    }

    fn price(&self, method: &str) -> f64 {
        match method {
            "Monte Carlo" => self.monte_carlo_price(100000),
            "FEM" => self.fem_price(100),
            "BSM" => self.bsm_price(),
            _ => panic!("Invalid pricing method"),
        }
    }

    fn monte_carlo_price(&self, num_simulations: usize) -> f64 {
        let mut rng = rand::thread_rng();
        let mut payoff_sum = 0.0;
        for _ in 0..num_simulations {
            let z: f64 = rng.gen_range(-3.0..3.0);
            let stock_price = self.underlying_price * f64::exp((self.risk_free_rate - 0.5 * self.implied_volatility.powi(2)) * self.time_to_maturity + self.implied_volatility * f64::sqrt(self.time_to_maturity) * z);
            if stock_price > self.barrier_price {
                payoff_sum += 1.0;
            }
        }
        payoff_sum / num_simulations as f64 * (-self.risk_free_rate * self.time_to_maturity).exp()
    }

    fn fem_price(&self, num_elements: usize) -> f64 {
        let dt = self.time_to_maturity / num_elements as f64;
        let mut prices = vec![0.0; num_elements + 1];
        prices[0] = self.underlying_price;
        let mut rng = rand::thread_rng();
        for i in 1..=num_elements {
            let z: f64 = rng.gen_range(-3.0..3.0);
            prices[i] = prices[i - 1] * f64::exp((self.risk_free_rate - 0.5 * self.implied_volatility.powi(2)) * dt + self.implied_volatility * f64::sqrt(dt) * z);
        }
        let payoff = if prices[num_elements] > self.barrier_price { 1.0 } else { 0.0 };
        payoff * (-self.risk_free_rate * self.time_to_maturity).exp()
    }

    fn bsm_price(&self) -> f64 {
        let d1 = (self.underlying_price.ln() / self.barrier_price.ln() + 
                  (self.risk_free_rate + 0.5 * self.implied_volatility.powi(2)) * self.time_to_maturity) / 
                 (self.implied_volatility * self.time_to_maturity.sqrt());
        let d2 = d1 - self.implied_volatility * self.time_to_maturity.sqrt();
        
        let normal = Normal::new(0.0, 1.0).unwrap();
        let n_d2 = normal.cdf(d2);
        
        (-self.risk_free_rate * self.time_to_maturity).exp() * n_d2
    }
}

fn main() {
    let mut option = DigitalCallOption::new();
    option.get_inputs();
    println!("Choose the pricing method (Monte Carlo/FEM/BSM):");
    let method = read_string();
    let price = option.price(&method);
    println!("The price of the digital call option is: {}", price);
}

fn read_f64() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please enter a valid number")
}

fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
