fn main() {
    println!("Hello, world!");
    println!("{}", max_profit(vec![7, 1, 5, 3, 6, 4]));
    println!("{}", max_profit(vec![7, 6, 4, 3, 1]));
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let (mut lowest, mut highest, mut profit): (usize, usize, i32) = (0, 0, 0);

    for i in 0..prices.len() {
        if prices[i] < prices[lowest] {
            (lowest, highest) = (i, i);
        } else if prices[i] > prices[highest] {
            highest = i;
            let profit_temp = prices[highest] - prices[lowest];
            if profit_temp > profit {
                profit = profit_temp;
            }
        }

        // if price > highest {
        //     highest = price;
        //     profit = highest - lowest;
        // } else if (lowest == -1) || (price < lowest && (highest - price) > profit) {
        //     println!("{}, {}, {}", price, lowest, highest);
        //     lowest = price;
        //     highest = 0;
        //     profit = highest - lowest;
        // }
    }

    profit
}
