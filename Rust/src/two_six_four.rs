// LeetCode #264 - Ugly Number II
// Date: 2024-08-18
// Submission Status: Time Limit Exceeded
// Note: Passed 500 / 596 tests before timing out on input x=1352
pub fn nth_ugly_number(n: i32) -> i32 {
  struct UglyNumbers {
    ugly_numbers: std::collections::HashSet<i32>,
  }

  impl UglyNumbers {
    fn check_if_ugly(&mut self, x: i32) -> bool {
      if x == 1 {
          return true
      }

      if self.ugly_numbers.get(&x).is_some() {
        return true;
      }

      let is_ugly = (x % 2 == 0 && self.ugly_numbers.get(&(x / 2)).is_some())
        || (x % 3 == 0 && self.ugly_numbers.get(&(x / 3)).is_some())
        || (x % 5 == 0 && self.ugly_numbers.get(&(x / 5)).is_some());

      if is_ugly {
        self.ugly_numbers.insert(x);
      }
      return is_ugly
    }
  }

  let mut ugly_numbers = UglyNumbers{
      ugly_numbers: vec![1,2,3,4,5,6].into_iter().collect(),
  };
  let mut count: i32 = 1;
  let mut number = 1;
  while count < n {
    number = number + 1;
    if (ugly_numbers.check_if_ugly(number)) {
      count = count + 1
    }
  }
  return number
}