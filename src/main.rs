use colored::*;
use pager::Pager;

fn main() {
    Pager::with_pager("less -RFX").setup();
    println!("{}", "foo".red());
}
