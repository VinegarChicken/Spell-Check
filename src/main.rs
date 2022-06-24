use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::{BufRead, BufReader, Cursor};
use std::ops::Range;


fn hash<T: Hash + AsRef<str>>(s: T) -> u64{
    let mut d = DefaultHasher::new();
    s.hash(&mut d);
    d.finish()
}


fn compare_words(word1: String, word2: String) -> f32{
    let word1:Vec<_> = word1.chars().collect();
    let word2:Vec<_> = word2.chars().collect();
    let mut similar:f32 = 0.0;
    for (index, char) in word1.clone().into_iter().enumerate(){
        if index >= word2.len(){
            break;
        }
        if char == word2[index]{
            similar+=1.0;
        }
    }
    similar / word2.len() as f32


}

fn main() {
   // println!("{}", compare_words("w".to_string(), "wori".to_string()));
    let file = include_bytes!("../words.txt");
    let reader = Cursor::new(file);

    let mut wordmap:HashMap<u64, String> = HashMap::new();

    for line in reader.lines(){
        if let Ok(s) = line{
            let s = s.to_lowercase();
            let hash = hash(&s);
            wordmap.insert(hash, s);
        }
    }

    loop {
        println!("Is this a word: ");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let mut buffer = buffer.replace("\n", "");
        if let Some(word) = wordmap.get(&hash(buffer.clone())){
            println!("'{word}' is a valid word");
        }
        else{
            for i in wordmap.iter(){
                if compare_words(i.1.clone(), buffer.clone()) > 0.75{
                    println!("{}", i.1.clone());
                }
            }
        }
    }


   // println!("test {} testa {}", hash("test"), hash(&cmpword));

}
