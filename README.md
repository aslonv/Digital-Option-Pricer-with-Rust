# Digital Call Option Pricing

A Rust implementation for pricing European-style digital call options using Monte Carlo simulation and Finite Element Method (FEM).

## Overview

This software provides two pricing methods for digital call options:
1. Monte Carlo simulation
2. Finite Element Method (FEM)

## DigitalCallOption Struct

Represents a digital call option with the following properties:
- `underlying_price`: Current price of the underlying asset
- `strike_price`: Option's strike price
- `barrier_price`: Option's barrier price
- `implied_volatility`: Implied volatility of the underlying asset
- `time_to_maturity`: Time to option maturity

### Methods

- `new() -> DigitalCallOption`: Creates a new instance with default values
- `get_inputs(&mut self)`: Prompts user for input values
- `price(&self, method: &str) -> f64`: Calculates option price using specified method

## Pricing Methods

### Monte Carlo Simulation

```rust
monte_carlo_price(&self, num_simulations: usize) -> f64
