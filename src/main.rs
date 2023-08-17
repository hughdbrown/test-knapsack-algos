use std::time::Instant;

use knapsack_utils::{
    make_items,
    sum_values,
    sum_weights,
    //select_items,
    Item,
    SearchResult,
};
use branch_and_bound::branch_and_bound;
use dynamic_programming::solve_dp;
use exhaustive_search::exhaustive_search;
use rods_technique::rods_technique;
use prng::Prng;

const MIN_VALUE: u64 = 1;
const MAX_VALUE: u64 = 160;
const MIN_WEIGHT: usize = 4;
const MAX_WEIGHT: usize = 130;

type FuncPointer<'a> = &'a dyn Fn(&[Item], usize) -> Result<SearchResult, ()>;

fn main() {
    let mut prng = Prng::new();
    let funcs: Vec<(&str, FuncPointer, usize)> = vec![
        (&"exhaustive", &exhaustive_search, 10),
        (&"branch", &branch_and_bound, 28),
        (&"rods", &rods_technique, 39),
        (&"dynamic", &solve_dp, 10000),
    ];

    for (i, func) in funcs.iter().enumerate() {
        let item_count = func.2;
        let items = make_items(
            &mut prng, item_count, MIN_VALUE, MAX_VALUE, MIN_WEIGHT, MAX_WEIGHT,
        );
        let total_value = sum_values(&items);
        let total_weight = sum_weights(&items);
        let allowed_weight = total_weight / 2;

        println!("*** Parameters ***");
        println!("# items:        {}", item_count);
        println!("Total value:    {}", total_value);
        println!("Total weight:   {}", total_weight);
        println!("Allowed weight: {}", allowed_weight);
        println!();

        for (func_name, alg, _count) in &funcs[i..] {
            run_algorithm(alg, func_name, &items, allowed_weight);
        }
    }
}

// Run the algorithm. Display the elapsed time and solution.
fn run_algorithm(
    alg: &dyn Fn(&[Item], usize) -> Result<SearchResult, ()>,
    func_name: &str,
    items: &[Item],
    allowed_weight: usize,
) {
    let start = Instant::now();
    let result: Result<SearchResult, ()> = alg(items, allowed_weight);
    let duration = start.elapsed();

    println!("---------- {}: {}", func_name, items.len());
    println!("Elapsed: {:?}", duration);
    match result {
        Ok(solution) => {
            println!("Value: {} Weight: {}", solution.2, solution.1);
        }
        Err(_) => {
            println!("No solution found");
        }
    }
    println!();
}
