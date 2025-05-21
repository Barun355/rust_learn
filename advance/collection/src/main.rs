mod hashmap;
mod iterators;
mod iteradapter;

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

    iteradapter::main();

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