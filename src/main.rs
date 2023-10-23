extern crate rand;
extern crate rayon;
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use rand::Rng;

const DATASET_LENGHT:usize = 3_248_700;
const DECK_SIZE:usize = 52;

type Card = u8;
type Deck = [Card; DECK_SIZE];
type Dataset = Vec<Deck>;

// generic trait for other shuffling algorithms
trait ShufflingAlgorithm: Send + Sync {
    fn name(&self) -> &str;
    fn shuffle(&self, deck: &mut Deck);
}

struct Algorithm1;
struct Algorithm2;

impl  ShufflingAlgorithm for Algorithm1{
    fn name(&self) -> &str {
        "v1_bin_shuffle"
    }

    fn shuffle(&self, deck: &mut Deck){
        const BIN_COUNTS:usize = 6;
        let mut bins: Vec<Vec<Card>> = vec![Vec::new(); BIN_COUNTS];

        let mut rng = rand::thread_rng();
        for &card in deck.iter() {
            let random_bin = rng.gen_range(0..BIN_COUNTS);
            // put the cards in random bins
            bins[random_bin].push(card);
        }

        let mut deck_position = 0;
        // Reassemble the deck
        for bin in bins {
            for card in bin {
                deck[deck_position] = card;
                deck_position += 1;
            }
        }
    }

}


impl  ShufflingAlgorithm for Algorithm2{
    fn name(&self) -> &str {
        "v2_bin_shuffle"
    }

    fn shuffle(&self, deck: &mut Deck){
        const BIN_COUNTS:usize = 6;
        let mut bins: Vec<Vec<Card>> = vec![Vec::new(); BIN_COUNTS];

        let mut rng = rand::thread_rng();
        for &card in deck.iter() {
            let random_bin = rng.gen_range(0..BIN_COUNTS);
            // put the cards in random bins
            bins[random_bin].push(card);
        }

        let mut deck_position = 0;
        // Reassemble the deck
        for bin in bins {
            for card in bin {
                deck[deck_position] = card;
                deck_position += 1;
            }
        }
    }

}

fn generate_dataset(algorithm: &Box<dyn ShufflingAlgorithm>, runs: i32 ) -> Dataset{
    let mut dataset: Dataset = vec![[0; DECK_SIZE]; DATASET_LENGHT];
    for deck in dataset.iter_mut() { 
        // build out deck
        for i in 0..DECK_SIZE{
            deck[i] = i as Card;
        }
    }
    for _ in 1..=runs{
        for deck in dataset.iter_mut() {
            algorithm.shuffle(deck);
        }
    }

    dataset
}

fn write_to_file(dataset: Dataset, file_name: &String) -> std::io::Result<()>{
    let flatten_dataset: Vec<Card> = dataset.into_iter().flatten().collect();

    let mut file = File::create(&file_name)?;

    file.write_all(&flatten_dataset)?;
    
    Ok(())
}

fn main() {
    let algorithms: Vec<Box<dyn ShufflingAlgorithm>> = vec![
        Box::new(Algorithm1),
        // Box::new(Algorithm2),
    ];

    // each algo simulation runs in parallel
    algorithms.par_iter().for_each(|algorithm| {
        // i will be used as runs
        for runs in 1..=3 {
            let dataset = generate_dataset(algorithm, runs);
            let file_name = format!("{}-{}.bin", algorithm.name(), runs);
            match write_to_file(dataset, &file_name) {
                Ok(_) => println!("File {} written succesfully", file_name),
                Err(e) => eprintln!("Error writing to file {}: {}", file_name, e),

            }
        }

    });
}
