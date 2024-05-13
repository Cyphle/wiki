use std::collections::HashMap;

pub fn popularity_analysis(scores: Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..scores.len() - 1 {
        if scores[i] > scores[i + 1] {
            increasing = false;
        }
        if scores[i] < scores[i + 1] {
            decreasing = false;
        }
    }

    increasing || decreasing
}

pub fn product_popularity() {
    let mut products = HashMap::new();
    products.insert("product 1", vec![1, 2, 2, 3]);
    products.insert("product 2", vec![4, 5, 6, 3, 4]);
    for (product_id, popularity) in products {
        if popularity_analysis(popularity) {
            println!("{} popularity is increasing or decreasing", product_id);
        } else {
            println!("{} popularity is fluctuating", product_id)
        }
    }
}