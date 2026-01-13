use dmrc_rs::load_routes;

fn main() {
    let test = load_routes();

    println!("stats:");
    println!("- stops: {}", test.stop_ids.len());
    println!("- routes: {}", test.route_ids.len());

    println!("- total ps x pt: {}", test.journeys.len());

    println!(
        "- min plans: {:?}",
        test.journeys.values().map(|j| j.len()).min()
    );
    println!(
        "- max plans: {:?}",
        test.journeys.values().map(|j| j.len()).max()
    );

    let count = test
        .journeys
        .iter()
        .filter(|(_, plans)| plans.is_empty())
        .inspect(|(key, _)| {
            let key = key.to_native();
            let ps = key >> 16;
            let pt = key & (0x0000FFFF);

            println!(
                "{} -> {}",
                test.stop_ids[ps as usize], test.stop_ids[pt as usize]
            );
        })
        .count();

    println!("- unreachable: {count}");
}
