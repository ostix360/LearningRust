use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert("Bleu", 10);
    scores.insert("Rouge",15);



    let equipe = vec!["Bleu", "Rouge"];
    let score = vec![10, 15];

    let mut scores: HashMap<_, _> = equipe.into_iter().zip(score.into_iter()).collect();

    print_hash_map(&scores);

    scores.entry("Bleu").or_insert(12);
    scores.entry("Jaune").or_insert(4);

    print_hash_map(&scores);

    print_hash_map(&count_word("Hello you Hello world How are you today?"))
}


fn print_hash_map(hashmap: &HashMap<&str, i32>){
    println!("--- HashMap Printer ---");
    for (key, value) in hashmap {
        println!("{} : {} ", key, value);
    }
}

fn count_word(text: &str) -> HashMap<&str, i32>{
    let mut table = HashMap::new();

    for word in text.split_whitespace(){
        let compt = table.entry(word).or_insert(0);
        *compt += 1;
    }

    return table;
}
