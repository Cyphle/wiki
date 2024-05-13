use crate::find_employee::find_employee_with_no_meeting;
use crate::highest_price_stock::highest_price_stock;
use crate::product_popularity::product_popularity;
use crate::search_words::search_word;
use crate::working_hours::working_hours;

mod search_words;
mod product_popularity;
mod highest_price_stock;
mod find_employee;
mod working_hours;

fn main() {
    // Search word from groups
    search_word();
    // Product popularity
    product_popularity();
    // Highest price stock
    highest_price_stock();
    // Findind employee with no meeting
    find_employee_with_no_meeting();
    // Longest working hours
    working_hours();
}
