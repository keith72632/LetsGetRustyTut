use std::collections::HashMap;

fn main() {
    // ----Strangs----
    let s = String::from("Fuck off");
    let s2 = String::from(" bitch");
    let s3: String = s + &s2;

    println!("{}", s3);

    let s4 = String::from(" get out the way");
    let s5 = format!("{}{}", s3, s4);
    println!("{}", s5);

    for c in "hello".chars() {
        // println!("{}", c);
        print!("{}", c);
    }

    //----Hashes---
    let blue = String::from("blue");
    let yellow = String::from("yellow");

    let mut scores = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 11);

    let team_name = String::from("blue");
    let score = scores.get(&team_name);

    print!("Team {}: {:?}\n", team_name, score);

    for(key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let mut scores2 = HashMap::new(); 
    scores2.insert(String::from("blue"), 69);
    // override
    scores2.insert(String::from("blue"), 77);

    // to avoid unwanted overrides, uses .entry().or_insert()
    scores2.entry(String::from("yellow")).or_insert(30);
    scores2.entry(String::from("yellow")).or_insert(50);

    println!("{:?}", scores2);

    // Populate hash map with words
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // or_insert returns mutable reference to value
        println!("count before {}", count);
        *count += 1;
        println!("count after {}", count);
    }

    println!("{:?}", map);

}
