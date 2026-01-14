use std::{
    io::{self, Write},
    time::Duration,
};

use dmrc_rs::{load_routes, ArchivedJourney, ArchivedJourneys};
use rkyv::rend::u32_le;

fn main() {
    let data = load_routes();

    println!("Delhi Metro Journey Planner");
    println!("----------------------------\n");

    let source_idx = prompt_stop_selection("Enter source stop", data);
    let target_idx = prompt_stop_selection("Enter target stop", data);

    let key = ((source_idx as u32) << 16) | (target_idx as u32);
    let key = u32_le::from_native(key);

    match data.journeys.get(&key) {
        Some(plans) if !plans.is_empty() => {
            println!(
                "\nJourney from {} to {}:",
                data.stops[source_idx].name, data.stops[target_idx].name
            );
            print_journey(&plans[0], data);
        }
        _ => println!("\nNo journey found between these stops."),
    }
}

fn prompt_stop_selection(prompt: &str, data: &ArchivedJourneys) -> usize {
    loop {
        print!("\n{}: ", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            println!("Please enter a search term.");
            continue;
        }

        let results = data.fuzzy_search(input);

        if results.is_empty() {
            println!("No matches found. Try again.");
            continue;
        }

        let top_results: Vec<_> = results.into_iter().take(5).collect();

        println!("\nMatches:");
        for (i, (idx, _score)) in top_results.iter().enumerate() {
            println!("  [{}] {}", i + 1, data.stops[*idx].name);
        }

        print!("\nSelect [1-{}]: ", top_results.len());
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        if let Ok(n) = choice.trim().parse::<usize>()
            && n >= 1
            && n <= top_results.len()
        {
            let (idx, _) = top_results[n - 1];
            return idx;
        }

        println!("Invalid selection. Try again.");
    }
}

fn print_journey(journey: &ArchivedJourney, data: &ArchivedJourneys) {
    println!(
        "  Arrival: {} seconds\n",
        humantime::format_duration(Duration::new(journey.arrival.to_native() as _, 0))
    );
    println!("  Route:");

    for entry in journey.plan.iter() {
        let route_idx = entry.0.to_native() as usize;
        let stop_idx = entry.1.to_native() as usize;
        let route = &data.routes[route_idx];
        let stop = &data.stops[stop_idx];
        println!("    [{:>6}] {}", route.long_name, stop.name);
    }
}
