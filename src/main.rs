use std::io;

struct Stock {
    name: String, 
    price: f32,
    budget: f32,
    shares: f32
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
                panic!("Please enter a valid integer");
            }
        };
        return input_num;
    }

    let mut stocks: Vec<Stock> = Vec::new(); // Create an empty vector to store stock names
    
    for i in 0..num_of_stocks() {
        if total_budget < 0.0 {
            panic!("Budget exceeded. Exiting."); // Exit the loop if the budget is exceeded
        };

        println!("Enter stock name {}: ", i + 1);
        let mut input_name = String::new();
        io::stdin().read_line(&mut input_name).expect("Failed to read input");
        let name = input_name.trim().to_string();
        
        println!("Enter price for {}:", name);   
        let mut input_price = String::new();
        io::stdin().read_line(&mut input_price).expect("failed to read input");
        let price: f32 = input_price.trim().parse().expect("ivalid input, please enter a number");

        println!("Leftover Budget: {}", total_budget);
        println!("Enter budget for {}:", name);
        let mut input_budget = String::new();
        io::stdin().read_line(&mut input_budget).expect("Failed to read input");
        let budget: f32 = input_budget.trim().parse().expect("Invalid input, please enter a number");
        total_budget -= budget;
        println!("Leftover Budget: {}", total_budget);
        if total_budget < 0.0 {
            panic!("Budget is less then 100");
        };

        let stock = Stock {
            name,
            price,
            budget,
            shares: 0.0,
        };
        stocks.push(stock);
    }
    
    println!("Stocks:");
    for stock in &stocks {
       println!("Name: {}, Price: {}, Budget: {}%", stock.name, stock.price, stock.budget);
    }  

    println!("How much are you willing to invest?");
    let mut input_invest = String::new();
    io::stdin().read_line(&mut input_invest).expect("failed to read input");
    let mut investment: f32 = input_invest.trim().parse().expect("ivalid input, please enter a number"); 
    
    let mut total_price: f32 = 0.0;
    for stock in &mut stocks {
        total_price += stock.price;
        stock.shares += 1.0;
        println!("Name: {}, Price: {}, Budget: {}%, Shares: {}", stock.name, stock.price, stock.budget, stock.shares);
    }
    investment -= total_price;

    println!("investment left over: {}", investment);
    for stock in &mut stocks {
        let mut spent = investment * (stock.budget / 100.0);
        println!("spent: {}", spent);
        while spent >= stock.price {
            spent -= stock.price;
            stock.shares += 1.0;
        };
        println!("Name: {}, Price: {}, Budget: {}%, Shares: {}", stock.name, stock.price, stock.budget, stock.shares);
    }
}
