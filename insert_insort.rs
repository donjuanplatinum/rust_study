// 5 2 4 6 1 3 8 0
// index = 1 , arr[index] = 2 , 5 > 2 , swap 5 and 2 , -> 2 5 4 ...
// index = 2 , arr[index] = 4 , 5 > 4 , swap 4 and 5 , -> 2 4 5 ...
// ...
// -> 1 2 3 4 5 6 8 0 ;  index = len - 1 , arr[index] = 0 ,  8 > 0, swap 8 and 0 , -> 1 2 3 4 5 6 0 8 , 6 > 0 .... ,-> 0 1 2 3 4 5 6 8 

use rand::{Rng, SeedableRng};
use rand::distributions::Uniform;
use rand::rngs::StdRng;

fn generate_random_array() -> Vec<i32> {
    let mut rng = StdRng::from_entropy(); // 从系统熵中生成种子
    let range = Uniform::new(0, 100000); // 随机数范围
    let mut random_numbers = Vec::with_capacity(1000000);

    for _ in 0..10000000 {
        random_numbers.push(rng.sample(range));
    }

    random_numbers
}


fn insert_insort<T: std::cmp::PartialOrd>(arr: &mut[T]) {
    
    let len = (*arr).len();
    for index in 1..len {
	println!("already run {} \n", index );
        let mut i = index ;
        while i > 0 && arr[i - 1] >= arr[i] {   
            (*arr).swap(i,i - 1);
            i -= 1;
        }
    }
}


fn main() {
    let mut random_array = generate_random_array();
    let mut arr: [i32;5] = [1, 6, 8, 3, 2];
    let mut arr1: [i32;5] = arr.clone();
    insert_insort::<i32>(&mut random_array);
}


