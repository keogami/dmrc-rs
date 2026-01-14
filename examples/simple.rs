use dmrc_rs::load_routes;

fn main() {
    let test = load_routes();

    println!("stats:");
    println!("- stops: {}", test.stops.len());
    println!("- routes: {}", test.routes.len());

    println!("- total ps x pt: {}", test.journeys.len());

    println!(
        "- min plans: {:?}",
        test.journeys.values().map(|j| j.len()).min()
    );
    println!(
        "- max plans: {:?}",
        test.journeys.values().map(|j| j.len()).max()
    );
}
