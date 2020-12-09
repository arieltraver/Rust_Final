use std::fs;
use std::string::String;
use std::collections::HashMap;
use std::str;
use std::collections::LinkedList;
use crossbeam_utils::thread;
use std::sync::atomic;
use std::sync::Arc;
//use std::str::Chars;
//use std::slice;

struct LetterMap{
    letter: String,
    map: HashMap<String, i32>,
    key_list: LinkedList<String>,
}

 fn main() {
    let test_1 = String::from("test_1.txt");
    let test_2 = String::from("test_2.txt");
    let buff1 = prepare_buff(test_1);
    calculate_word_count(buff1);
}

fn prepare_buff(filename:String) -> String{
    let mut buff:String = fs::read_to_string(filename)
        .expect("bad file read");
    buff = replace_chars(buff);
    buff = str::to_lowercase(&buff[..]);
    buff
}

fn hash_words(mut lmap: LetterMap, letter:String, buff:String){//->LetterMap{
    // splits the word buffer by white spaces 
    // the method also creates a iterator
    let letter_struct = &mut lmap;
    let cursor = buff.split_whitespace();
    
    for current in cursor {
        // for each word in iterator that starts with the 
        if current.starts_with(&letter) {
           if letter_struct.map.contains_key::<str>(&current) {
               let count=letter_struct.map.get_mut::<str>(&current).unwrap();
               *count = *count+1;
           }
           else { letter_struct.map.insert( (&current).to_string(), 1);}
        }
    }
    //*letter_struct
} 
fn calculate_word_count(buff:String){

    let alphabet2 = "abcdefjhijklmnopqrsuvwxyz".chars();
    let mut alphabet = Vec::with_capacity(26);
    for ch in alphabet2 {
        let mut chs = ch.to_string();
        alphabet.push(chs);
        let mut chs2 = ch.to_string();
        let mut hmap = HashMap::<String,i32>::new();
        let mut llist = LinkedList::<String>::new();
        let mut lmap = LetterMap{letter:chs2,map:hmap,key_list:llist};
    }
    let mut i = 0;
    crossbeam::scope(|s| { //make a new thread for every letter of the alphabet
        while (i < 26) { 
            let handle = s.spawn(|_| {
                let mut hmap = HashMap::<String,i32>::new();
                let mut llist = LinkedList::<String>::new();
                let mut lmap = LetterMap{letter:alphabet[i],map:hmap,key_list:llist};
                let cursor = buff.split_whitespace();
				for current in cursor {
        // for each word in iterator that starts with the 
					if current.starts_with(&alphabet[i]) {
                
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
            i += 1;
        }
    });
}

/**
 fn run_tests(filename:String){

    let buff:String = prepare_buff(filename);
    println!("{}", buff); 
    let v = calculate_word_count(buff);

    for lmap in v {
        for (word, count) in lmap.map.iter() {
            println!("the word count for {}: {}",word,count);
        }
    }

}
**/
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