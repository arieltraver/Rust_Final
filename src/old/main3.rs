#[macro_use]
extern crate lazy_static;
use std::time::{Duration, Instant};
use std::fs;
use std::string::String;
use std::str;
use std::sync::RwLock;
use std::sync::Arc;
//use fnv::FnvHashMap;
//use std::hash::{Hash, Hasher};
use std::collections::HashMap;

/**
 * LetterMap
 * A hashmap and the letter which all of its word keys start with
 * Used by the multithreaded version
 * **/
struct LetterMap{
    letter: String,
    map: HashMap<String, i32>,
}

lazy_static! {
    // creates a global instance of the text which can be shared with threads 
    pub static ref TEXT_LOCK: RwLock<String> = RwLock::new(String::new());
}

fn main() {
    run_tests();
}



 fn run_tests() {
    /**
    let test_1 = String::from("test_2.txt");
    prepare_buff(test_1);
    
    let start = Instant::now();
    calculate_word_count(7);
    let mut elapsed = start.elapsed();
    
    let start2 = Instant::now();
    calculate_word_single();
    let mut elapsed2 = start2.elapsed();

    let test_2 = String::from("mobydick.txt");
    prepare_buff(test_2);
    
    let start3 = Instant::now();
    calculate_word_count(13);
    let elapsed3 = start3.elapsed();
    
    let start4 = Instant::now();
    calculate_word_single();
    let elapsed4 = start4.elapsed();
    
    **/
    let test_3 = String::from("./text/mobydick.txt");
    prepare_buff(test_3);

    let start5 = Instant::now();
    calculate_word_count(7);
    let elapsed5 = start5.elapsed();

    let start6 = Instant::now();
    calculate_word_single();
    let elapsed6 = start6.elapsed();
    /**
    println!("Time elapsed without concurrency: {:?}", elapsed2);
    println!("Time elapsed with concurrency: {:?}", elapsed);
    println!();
    println!("Time elapsed without concurrency: {:?}", elapsed4);
    println!("Time elapsed with concurrency: {:?}", elapsed3);
    println!();
    **/
    println!("Time elapsed without concurrency: {:?}", elapsed6);
    println!("Time elapsed with concurrency: {:?}", elapsed5);
    }

fn prepare_buff(filename:String){
    let mut buff:String = fs::read_to_string(filename)
        .expect("bad file read");
    buff = replace_chars(buff);
    buff = str::to_lowercase(&buff[..]); 
    let mut text = TEXT_LOCK.write().unwrap();
    *text = buff;    
}

fn calculate_word_single() {
    let mut map = HashMap::<String, i32>::new();
    let buff = TEXT_LOCK.read().unwrap();
    let cursor = buff.split_whitespace();
    for current in cursor {
   		if map.contains_key::<str>(&current) {
            let count=map.get_mut::<str>(&current).unwrap();
            *count = *count+1
			}
        else {map.insert((&current).to_string(), 1);
    	}
    }
	for (key, count) in map.iter() {
		println!("The word count for {}:{}", key, count);
	} 
}

fn calculate_word_count(range0:u8){
    let mut i:u8 = 97; //u8 for the character 'a'
    crossbeam::scope(|s| { //threads guaranteed to join before this scope ends
    while i <= 122 { //u8 for the character 'z'
        let icopy = Arc::new(i);
        let range = Arc::new(range0);
        s.spawn(move |_| {
            let chs = (*icopy as char).to_string(); //for use in string checking
            let chs2 = (*icopy as char).to_string(); //it requires two
            let mut hmap = HashMap::<String,i32,>::new(); //new map for thread
            //let mut hmap = HashMap::<String, i32, BuildHasherDefault<FnvHashMap>>::default();
            let buff = TEXT_LOCK.read().unwrap(); // acquire the lock for reading
            let mut lmap = LetterMap{letter:chs,map:hmap};
            let cursor = buff.split_whitespace(); //break by spaces
            for current in cursor {
                let first = current.chars().next().unwrap() as u8;
                if first >= *icopy && first < *icopy + *range { //only look at certain letters
                    if lmap.map.contains_key::<str>(&current) {
                        let count=lmap.map.get_mut::<str>(&current).unwrap();
                        *count = *count+1;
                    }
                    else { lmap.map.insert((&current).to_string(), 1);
                    }
                }
            }
            for (word, count) in lmap.map.iter() {
                println!("the word count for {}: {}",word,count);
            }
        });
        i += range0;
    }
}).unwrap();
}


fn replace_chars(buff:String)->String {
    let v = vec![',','\"','.','!','?','(',')','{','}',':',';','。','，','\\', '/','_','“','”'];
    let mut new_buff = String::new();
    for ch in buff.chars() {
        if !v.contains(&ch) {
            new_buff.push(ch);
        }
        else if ch == '\n' {
            new_buff.push(' ');
        }
    }
    new_buff
} 
