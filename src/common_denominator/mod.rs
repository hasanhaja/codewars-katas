use std::fmt::{ self , Display, Formatter};

/// # Common Denominator Kata
/// This is a solution for the Codewars Kata: https://www.codewars.com/kata/54d7660d2daf68c619000d95/train/rust
/// I have taken this opportunity to finally and properly learn quicksort effectively. It keeps slipping my mind.
///
/// # Approach
/// I haven't solved the problem yet, but this is the series of steps I have in mind:
/// * Convert vector of tuples to vector of Fractions
/// * Sort the vector by the denominator in reverse order so the left most fraction will have the largest denominator
/// * Check if it is reducible and if it is reduce and move to the next element
/// * Sort it again
/// * Multiple the all denominators except the largest one to name all the same
/// * Make sure even though the sorting and things were happening with regards to the denominator, the numerators were also being updated.
/// * TODO I'm not sure beyond this

// TODO Impl mathematical ops: https://doc.rust-lang.org/rust-by-example/trait/ops.html
// TODO Impl ordering in terms of denominator: https://www.philipdaniels.com/blog/2019/rust-equality-and-ordering/
struct Fraction {
    num: i64,
    den: i64,
}

impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}/{})", self.num, self.den)
    }
}

impl Fraction {
    pub fn new(frac: (i64, i64)) -> Fraction {
        Fraction {
            num: frac.0,
            den: frac.1,
        }
    }

    pub fn value(&self) -> f64 {
        let num = self.num as f64;
        let den = self.den as f64;
        num / den
    }
}

pub fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    // your code
    let list_of_fracs: Vec<Fraction> = l.iter().map(|frac| Fraction::new(*frac)).collect();

    todo!("Sort the vector by the denominator");

    Vec::new()
}

/// Mostly used https://en.wikipedia.org/wiki/Quicksort for reference of the
/// Lomuto partition scheme. I want to try to implement:
/// TODO Random pivot instead of the last element
/// TODO With slices instead of a vector even though a vector makes more sense for my use case.
/// TODO take a vector and return another for learning purposes
fn quicksort(nums: &mut Vec<i64>, low: usize, high: usize) {
    if low < high {
        let part = partition(nums, low, high);
        quicksort(nums, low, part-1);
        // TODO are we doing part - 1 and part + 1 because the part is the right position?
        quicksort(nums, part + 1, high);
    }
}

fn partition(nums: &mut Vec<i64>, low: usize, high: usize) -> usize {
    let pivot = nums[high];
    let mut i = low;
    println!("Entering partition as: {:?}", nums);
    for j in low..high {
        println!("{:?}", nums);
        if nums[j] <= pivot {
            swap(nums, i, j);
            i += 1
        }
    }
    swap(nums, i, high);
    println!("Leaving partition as: {:?}", nums);
    i
}

fn swap(arr: &mut Vec<i64>, i: usize, j: usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

#[cfg(test)]
mod test {
    use super::{ convert_fracts, quicksort };
    use crate::common_denominator::Fraction;

    fn testing(l: Vec<(i64, i64)>, exp: Vec<(i64, i64)>) -> () {
        assert_eq!(convert_fracts(l), exp)
    }

    #[test]
    fn basics_convert_fracts() {

        testing(vec![(69, 130), (87, 1310), (3, 4)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
        testing(vec![(690, 1300), (87, 1310), (30, 40)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);

    }

    #[test]
    fn quicksort_sorts_list_of_nums() {
        let mut nums = vec![9, 4, 1, 3];
        let length = nums.len();
        quicksort(&mut nums, 0usize, length - 1);
        assert_eq!(nums, vec![1, 3, 4, 9]);
    }

    #[test]
    fn quicksort_sorts_list_of_repeating_nums() {
        let mut nums = vec![9, 4, 1, 3, 1, 1];
        let length = nums.len();
        quicksort(&mut nums, 0usize, length - 1);
        assert_eq!(nums, vec![1, 1, 1, 3, 4, 9]);
    }

    #[test]
    fn quicksort_sorts_list_of_more_repeating_nums() {
        let mut nums = vec![9, 4, 1, 3, 1, 1, 9, 4, 1, 3];
        let length = nums.len();
        quicksort(&mut nums, 0usize, length - 1);
        assert_eq!(nums, vec![1, 1, 1, 1, 3, 3, 4, 4, 9, 9]);
    }

    #[test]
    fn compute_fraction_value() {
        let frac = Fraction::new((1, 2));
        let result = frac.value();
        assert_eq!(result, 0.5);
    }
}