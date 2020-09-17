fn main() {
    let list = vec![2, 5, 3, 4, 1];
    let sorted_list_bubble = bubble_sort(list.clone());
    println!("Sorted List: {:?}", sorted_list_bubble);

}

fn bubble_sort(mut input_list: Vec<usize>) -> Vec<usize> {
    let n = input_list.len();
    for i in 0..(n-1) {
        for j in 0..(n - i - 1) {
            if input_list[j] > input_list[j + 1] {
                let t = input_list[j];
                input_list[j] = input_list[j + 1];
                input_list[j + 1] = t;
            }
        }
    }
    input_list
}



