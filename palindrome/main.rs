fn main() {
  println!("{}", is_palindrome("Hello, world!"));
  println!("{}", is_palindrome("abcdedcba"));
}

pub fn is_palindrome(s: &str) -> bool {
  let reversed = reverse(s);
  s == reversed
}

pub fn reverse(s: &str) -> String {
  s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_reverse() {
      assert_eq!(reverse("hello"), "olleh");
      assert_eq!(reverse("rust"), "tsur");
      assert_eq!(reverse(""), "");
  }

  #[test]
  fn test_is_palindrome() {
      assert!(is_palindrome("madam"));
      assert!(is_palindrome("racecar"));
      assert!(is_palindrome("a"));
      assert!(is_palindrome(""));
      assert!(!is_palindrome("hello"));
      assert!(!is_palindrome("rust"));
  }
}
