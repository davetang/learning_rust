use std::io::BufReader;
use std::fs::File;
use bio::io::fasta;

fn main() {
    let f = File::open("data/test.fa").unwrap();
    let reader = fasta::Reader::new(BufReader::new(f));

    let mut entries = 0;

    for result in reader.records() {
        let record = result.expect("Error during FASTA record parsing");
        entries += 1;
        let nb_bases = record.seq().len();
        println!("ID: {}", record.id());
        println!("Description: {}", record.desc().unwrap());
        println!("Number of bases: {}", nb_bases);
    }

    println!("Total number of entries: {}", entries);
}
