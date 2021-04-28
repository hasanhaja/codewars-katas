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

fn quicksort(nums: Vec<i64>) -> Vec<i64> {
    todo!("Implement quicksort first to see how that can map to a Vec<Fraction>");
    Vec::new()
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
        let nums = vec![9, 4, 1, 3];
        let sorted = quicksort(nums);
        assert_eq!(sorted, vec![1, 3, 4, 9]);
    }

    #[test]
    fn compute_fraction_value() {
        let frac = Fraction::new((1, 2));
        let result = frac.value();
        assert_eq!(result, 0.5);
    }
}