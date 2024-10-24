# My Finance App

Rust application that's using [Slint](https://slint.rs/) for Finance Management App.
 
### Try it yourself => [Online simulation](https://qubi0-0.github.io/My-Finance/)

## About

I want to make an app, which allows you to calculate your expenses, including onetime purchases and monthly payments. 

## Usage (or Modifying)

1. Install Rust by following its [getting-started guide](https://www.rust-lang.org/learn/get-started).
   Once this is done, you should have the `rustc` compiler and the `cargo` build system installed in your `PATH`. 
2. Build with `cargo`:
    ```
    cargo build
    ```
3. Run the application binary:
    ```
    cargo run
    ```


# Visualization

## Calculate Tab
Allows to quickly check how much does something cost you depending how often you spend specific amount of money.

![alt text](images/Calculate_ui.png)

## Add New Tab
Allows to add new expenses to see total cost (in future there will be (daily,monthly and ect) functionalities added)

![add new](images/addnew_ui.png)

## Fuel Cost Tab 
Allows to quickly estimate fuel cost for your trip

![fuel cost](images/fuel_ui.png)