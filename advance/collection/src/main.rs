mod hashmap;
mod iterators;
mod iteradapter;
mod string_slice;

fn main(){
    // let mut vect = Vec::new();

    // for i in 1..100 {
    //     vect.push(i);
    // }

    // println!("{:?}", vect);
    // let filter_even = find_even(&vect);

    // println!("{:?} \n{:?}", filter_even, vect)
    
    // find_even_1(&mut vect);
    
    // println!("After Filter: {:?}", vect);

    // hashmap::main();

    // iterators::main();

    // iteradapter::main();

    // Strings
    // let name = String::from("hello world"); // String type
    // let name_slice = &name; // Has a `view` into the original string is a reference
    // let string_literal = "hello world"; // literal is also an &str but it points to directly to an address in the binary.
    // string_slice::main();

    // string slice
    let sentence = String::from("Hello World, Barun Tiwary");

    let sentence_first_word = string_slice::first_word(&sentence);
    println!("{}", sentence_first_word);
}


fn find_even(vec: &Vec<i32>) -> Vec<i32> {

    let mut new_vec = Vec::new();


    for item in vec {
        if item % 2 == 0 {
            new_vec.push(*item);
        }
    }

    return new_vec;
}

fn find_even_1(vec: &mut Vec<i32>) {
    
    let mut i = 0;
    while i < vec.len() {
        if vec[i] % 2 != 0 {
            vec.remove(i);
        }

        i = i + 1;
    }

}