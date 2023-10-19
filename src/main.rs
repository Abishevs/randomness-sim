extern crate rand;
use std::fs::File;
use std::io::Write;

use rand::Rng;

const DATASET_LENGHT:usize = 3_248_700;
const DECK_SIZE:usize = 52;

type Card = u8;
type Deck = [Card; DECK_SIZE];
type Dataset = Vec<Deck>;

fn bin_shuffle(deck: &mut Deck){
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

fn generate_dataset(shuffle_algo: fn(&mut Deck)) -> Dataset{
    let mut dataset: Dataset = vec![[0; DECK_SIZE]; DATASET_LENGHT];
    for deck in dataset.iter_mut() { 
        // build out deck
        for i in 0..DECK_SIZE{
            deck[i] = i as Card;
        }
    }

    for deck in dataset.iter_mut() {
        shuffle_algo(deck);
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

    for i in 0..3 {
        let dataset = generate_dataset(bin_shuffle);
        let file_name = format!("test_bin_shuffle-{}.bin", i);
        match write_to_file(dataset, &file_name) {
            Ok(_) => println!("File {} written succesfully", file_name),
            Err(e) => eprintln!("Error writing to file {}: {}", file_name, e),
            
        }
     }
}
