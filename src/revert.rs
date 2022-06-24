//println!("a {} b {}", 'a' as u8, 'b' as u8);
//println!("testa {}", );

let file = include_bytes!("../words.txt");
let test = String::from("testa");
println!("{}", str_to_asciinum(test));
let reader = Cursor::new(file);
let mut wordmap:HashMap<u32, Vec<String>> = HashMap::new();

for line in reader.lines(){
if let Ok(s) = line{
let s = s.to_lowercase();

}
}
/*
    let mut wordmap:HashMap<u32, Vec<String>> = HashMap::new();

    for line in reader.lines(){
        if let Ok(s) = line{
            let s = s.to_lowercase();
            let hash = str_to_asciinum(s.clone());
            if let Err(e) = wordmap.try_insert(hash, vec![s]){
               // println!("{e} {:?} {}", , )
                let key = e.entry.key().clone();
                let mut value = e.value.clone();
                let mut words = wordmap.get(&key).unwrap().clone();
                words.append(&mut value);
                wordmap.insert(key, words.clone());
                println!("insert {:?}", words.clone());
            }
        }
    }
    println!("{:?}", wordmap.get(&str_to_asciinum(String::from("test"))));

 */