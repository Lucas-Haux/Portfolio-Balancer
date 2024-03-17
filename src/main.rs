use std::io;
i
use std::process::abort;

struct Stock {
    name: String, 
    price: i32,
    budget: f32,
}

fn main() {

    let mut total_budget: f32 = 100.0;

    fn num_of_stocks() -> i32 { 
        println!("How many stocks do you want to enter");
        let mut input_str = String::new();
        io::stdin().read_line(&mut input_str).expect("Failed to read input");

        let input_num: i32 = match input_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid integer");
                return 0;
            }
        };
        return input_num;
    }

    let mut stocks: Vec<Stock> = Vec::new(); // Create an empty vector to store stock names
    
    for i in 0..num_of_stocks() {
        println!("Enter stock name {}: ", i + 1);
        let mut input_name = String::new();
        io::stdin().read_line(&mut input_name).expect("Failed to read input");
        let name = input_name.trim().to_string();
        
        println!("Enter price for {}:", name);   
        let mut input_price = String::new();
        io::stdin().read_line(&mut input_price).expect("Failed to read input");
        let price: i32 = input_price.trim().parse().expect("Ivalid input, please enter a number");

        println!("Leftover Budget: {}", total_budget);
        println!("Enter budget for {}:", name);
        let mut input_budget = String::new();
        io::stdin().read_line(&mut input_budget).expect("Failed to read input");
        let budget: f32 = input_budget.trim().parse().expect("Invalid input, please enter a number");
        total_budget -= budget;
        println!("Leftover Budget: {}", total_budget);

        if i > 0 && total_budget < 0.0 {
            panic!println!("Budget exceeded. Exiting."); // Exit the loop if the budget is exceeded
        };

        let stock = Stock {
            name,
            price,
            budget,
        };
        stocks.push(stock);
    }
    
    println!("Stocks:");
    for stock in &stocks {
       println!("Name: {}, Price: {}, Budget: {}%", stock.name, stock.price, stock.budget);
    }  
}
