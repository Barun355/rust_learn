use std::collections::HashMap;

fn group_values_by_key(vec: Vec<(String, i32)>) -> HashMap<String, Vec<i32>> {

    let mut new_vec: HashMap<String, Vec<i32>> = HashMap::new();

    let mut vec_iter = vec.iter();

    while let Some((key, value)) = vec_iter.next() {
        if !new_vec.contains_key(key) {
            new_vec.insert(key.to_string(), vec![*value]);
        } else {
            new_vec.entry(key.to_string()).and_modify(|hm_value|  hm_value.push(*value));
        }
    }

    // for (key, value) in vec {
    //     if !new_vec.contains_key(&key) {
    //         new_vec.insert(key, vec![value]);
    //     } else {
    //         new_vec.entry(key).and_modify(|hm_value| hm_value.push(value));
    //     }
    // }

    return  new_vec;
}


pub fn main(){

    let pairs: Vec<(String, i32)> = vec![
        (String::from("a"), 20),
        (String::from("b"), 30),
        (String::from("a"), 40),
        (String::from("b"), 10)
    ];

    let group_values = group_values_by_key(pairs);

    for (key, value) in group_values {
        println!("{} = {:?}", key, value);
    }
}