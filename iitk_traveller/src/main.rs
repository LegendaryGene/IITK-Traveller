mod lexer;
mod traveller;

use std::env;

fn main() {
    // Read user's code file.
    let args: Vec<String> = env::args().collect();
    let filename = match args.len() {
        2 => &args[1],
        _ => panic!("Invalid parameters. Usage: ./interpreter <filename>!"),
    };

    let mut mem: Vec<Vec<i32>> = vec![vec![0; 2048]]; // Initialise memory.
    let mut mem_flag: Vec<Vec<i8>> = vec![vec![0; 2048]]; // To check for EOL character.
    let (tokens, lines) = lexer::store_input(filename); // Apply lexer.
    let locations = lexer::create_map(); // Create a map of all possible locations.
    let (graph, increment_graph) =
        lexer::build_graph(&tokens, &locations, lines); // Parse the code.
    let mut traveller = traveller::TravelStat::new(0, 0, 1, 0, 2, 0, 0, 0, 0); // Initialise Traveller.
    traveller.travel(
        &mut mem,
        &mut mem_flag,
        &locations,
        &graph,
        &increment_graph,
    );
}
