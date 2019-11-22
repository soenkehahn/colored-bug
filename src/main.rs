use colored::*;
use pager::Pager;

fn main() {
    Pager::new().setup();
    println!("{}", "foo".red());
}
