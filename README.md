# digital call option pricing

this piece of software implements pricing of a digital CALL option,
the option observation type being European. it supports two pricing methods:
- monte carlo simulation
- finite element method (fem)

### DigitalCallOption

the struct represents a digital call option with the following fields:

- **underlying_price**: The current price of the underlying asset.
- **strike_price**: The strike price of the option.
- **barrier_price**: The barrier price of the option.
- **implied_volatility**: The implied volatility of the underlying asset.
- **time_to_maturity**: The time to maturity of the option.

#### methods

- **new() -> DigitalCallOption**: creates a new instance of `DigitalCallOption` with all fields initialized to 0.0.

- **get_inputs(&mut self)**: prompts the user to input the values for each field of the `DigitalCallOption`.

- **price(&self, method: &str) -> f64**: calculates the price of the option based on the provided pricing method. It supports "Monte Carlo" and "FEM" methods

## pricing methods

### monte carlo simulation

calculates the price of the option using mc simulation.

- **monte_carlo_price(&self, num_simulations: usize) -> f64**: takes the number of simulations as an argument.

### finite element method

calculates the price of the option using the Finite Element Method.

- **fem_price(&self, num_elements: usize) -> f64**: Takes the number of elements as an argument.

### how to use the program

- cargo build

- cargo run