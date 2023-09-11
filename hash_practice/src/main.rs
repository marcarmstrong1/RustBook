fn main() {
    //Start with list of integers and return mean, median, and mode 
    let mut list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    //Mean
    let mut sum = 0;
    for i in &list {
        sum += i;
    }

    let mean = sum / list.len();
    println!("Mean: {}", mean);

    //Median
    let mut median = 0;
    let mut sorted_list = list.clone();
    sorted_list.sort();
    let mid = sorted_list.len() / 2;
    if sorted_list.len() % 2 == 0 {
        median = (sorted_list[mid] + sorted_list[mid - 1]) / 2;
    } else {
        median = sorted_list[mid];
    }

    println!("Median: {}", median);

    //Mode
    let mut mode = 0;
    let mut count = 0;
    let mut max_count = 0;
    let mut mode_list = list.clone();
    mode_list.sort();
    for i in &mode_list {
        if *i == mode {
            count += 1;
        } else {
            count = 1;
        }
        if count > max_count {
            max_count = count;
            mode = *i;
        }
    }
    if mode == 0 {
        println!("Mode: None");
    } else {
        println!("Mode: {}", mode);
    }
}








