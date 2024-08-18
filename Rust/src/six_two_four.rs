// LeetCode #624 
// Date: 2024-08-16
pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
    fn get_min_max(arrays: &Vec<Vec<i32>>, skip_index: Option<usize>) -> ((usize, i32), (usize, i32)) {
        let min = arrays.iter().enumerate().fold((0, i32::MAX), |acc, (i, x)| {
            if skip_index.is_some_and(|x| x == i) {
                return acc
            }
            if x[0] < acc.1 {
                return (i, x[0]);
            }
            return acc;
        });
        let max = arrays.iter().enumerate().fold((0, i32::MIN), |acc, (i, x)| {
            if skip_index.is_some_and(|x| x == i) {
                return acc
            }
            if x.last().expect("Test constrains says that there should not be empty arrays") > &acc.1 {
                return (i, *x.last().expect("Test constrains says that there should not be empty arrays"));
            }
            return acc;
        });
        return (min, max)
    }

    let (min, max) = get_min_max(&arrays, None);
    if min.0 == max.0 {
        let (second_min, second_max) = get_min_max(&arrays, Some(min.0));
        return std::cmp::max(max.1 - second_min.1, second_max.1 - min.1);
    } else {
        return max.1 - min.1;
    }
}