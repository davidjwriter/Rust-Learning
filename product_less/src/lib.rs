pub fn findSubarrays(arr: &[u32], target: u32) -> Vec<Vec<u32>> {
    let mut subarrays = Vec::new();
    // [2, 5, 3, 10]
    // [[2], [5], [2, 5], [3], [5, 3], [10]]
    let mut i = 0;
    let mut j = 0;
    let mut curr_sum = 1;
    while i < arr.len() {
        curr_sum *= arr[j];
        while curr_sum < target && j + 1 < arr.len() {
            j += 1;
            curr_sum *= arr[j];
        }
        print!("curr sum: {}\n", curr_sum);
        print!("j is: {}\n", j);
        let mut temp_arr: Vec<u32> = Vec::new();
        for index in i..j {
            print!("\nhere\n");
            temp_arr.push(arr[index]);
            subarrays.push(temp_arr.clone());
        }
        i += 1;
        curr_sum = 1;
        j = i;
    }
    subarrays
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one() {
        let arr: [u32; 4] = [2, 5, 3, 10];
        let mut answer: Vec<Vec<u32>> = Vec::new();
        answer.push(vec![2]);
        answer.push(vec![5]);
        answer.push(vec![2, 5]);
        answer.push(vec![3]);
        answer.push(vec![5, 3]);
        answer.push(vec![10]);
        let test = findSubarrays(&arr, 30);
        assert_eq!(test, answer);
    }

    #[test]
    fn test_two() {
        let arr: [u32; 4] = [8, 2, 6, 5];
        let mut answer: Vec<Vec<u32>> = Vec::new();
        answer.push(vec![8]);
        answer.push(vec![2]);
        answer.push(vec![8, 2]);
        answer.push(vec![6]);
        answer.push(vec![2, 6]);
        answer.push(vec![5]);
        answer.push(vec![6, 5]);
        let test = findSubarrays(&arr, 50);
        assert_eq!(test, answer);
    }
}
