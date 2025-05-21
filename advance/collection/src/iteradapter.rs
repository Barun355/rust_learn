pub fn main() {
    consuming_adapter();
    iterator_adapter();


    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let filtered_vec = odd_double_new_vec(vec);

    println!("{:?}", filtered_vec);
}

fn consuming_adapter() {
    let vec = vec![1, 2, 3, 4];
    println!("{:?}", vec);

    let vec_iter = vec.iter();

    let sum: i32 = vec_iter.sum();

    println!("{}", sum);
}

fn iterator_adapter() {

    let v1 = vec![1, 2, 3, 4];

    let v1_iter = v1.iter();

    // let v1_iter_1 = v1_iter.map(|x| x * x);

    // for item in v1_iter_1 {
    //     println!("{}", item)
    // }

    let odd_iter = v1_iter.filter(|x| **x % 2 != 0);

    for item in odd_iter {
        println!("{}", item);
    }

}

fn odd_double_new_vec(vec: Vec<i32>) -> Vec<i32> {
    let iter = vec.iter();

    let odd_iter = iter.filter(|x| *x % 2 != 0 );

    let double_iter = odd_iter.map(|x| *x * 2);

    let mut final_vec = vec![];

    for item in double_iter {
        final_vec.push(item);
    }

    return  final_vec;

    // return  vec.iter().filter(|x| *x % 2 != 0 ).map(|x| *x * 2).collect();
}