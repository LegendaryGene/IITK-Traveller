mod lexer;
mod traveller;

use std::env;

fn main() {
    // Read user's code file.
    let args: Vec<String> = env::args().collect();
    let filename = match args.len() {
        2 => &args[1],
        _ => {
            println!("Invalid parameters. Usage: ./interpreter <filename>!");
            return;
        }
    };

    let mut mem: Vec<Vec<i32>> = vec![vec![0; 2048]]; // Initialise memory.
    let mut mem_flag: Vec<Vec<i8>> = vec![vec![0; 2048]]; // To check for EOL character.
    let (tokens, lines) = match lexer::store_input(filename) {
        Ok((tokens, lines)) => (tokens, lines),
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    }; // Apply lexer.
    let locations = lexer::create_map(); // Create a map of all possible locations.
    let (graph, increment_graph) =
        match lexer::build_graph(&tokens, &locations, lines) {
            Ok((graph, increment_graph)) => (graph, increment_graph),
            Err(e) => {
                println!("Error: {}", e);
                return;
            }
        }; // Parse the code.
    let mut traveller = traveller::TravelStat::new(
        0,
        0,
        1,
        0,
        2,
        0,
        0,
        0,
        0,
        &mut mem,
        &mut mem_flag,
    ); // Initialise Traveller.
    match traveller.travel(&locations, &graph, &increment_graph, 0) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
}
