mod lexer;
mod traveller;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.len() {
        2 => &args[1],
        _ => panic!("Invalid parameters. Usage: ./interpreter <filename>!"),
    };

    let mut mem = vec![vec![0; 2048]];
    let (tokens, lines) = lexer::store_input(filename);
    let locations = lexer::create_map();
    let graph = lexer::build_graph(&tokens, &locations, lines);
    let mut traveller = traveller::TravelStat::new(0, 0, 1, 0, 2, 0, 0, 0);

    traveller.travel(&mut mem, &locations, &graph);
}
