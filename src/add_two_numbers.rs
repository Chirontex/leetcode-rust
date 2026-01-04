// https://leetcode.com/problems/add-two-numbers
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  #[allow(dead_code)]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

#[allow(dead_code)]
impl Solution {
    // TODO: "1567 / 1569 testcases passed", so must keep working on it
    // TODO: move private methods to a trait implementation
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::decompile_number(
            Self::compile_number(&l1) + Self::compile_number(&l2)
        )
    }

    fn compile_number(list : &Option<Box<ListNode>>) -> u128 {
        let mut result: u128 = 0;
        let mut med: Vec<i32> = vec![];
        let mut current_node = list;

        while let Some(node) = current_node {
            med.push(node.val);

            current_node = &node.next;
        }

        if med.len() > 0 {
            med.reverse();

            for (i, &num) in med.iter().enumerate() {
                result += num as u128 * 10u128.pow((med.len() - i - 1) as u32);
            }
        }

        result
    }

    fn decompile_number(number: u128) -> Option<Box<ListNode>> {
        let mut med: Vec<i32> = vec![];

        for num in number.to_string().chars() {
            med.push(String::from(num).parse().unwrap());
        }

        let mut previous: Option<Box<ListNode>> = None;

        if med.len() > 0 {
            for &num in med.iter() {
                previous = Some(Box::new(ListNode {
                    val: num,
                    next: previous,
                }));
            }
        }

        previous
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_two_numbers() {
        // TODO: shortify it
        let examples = vec![
            (
                Some(Box::new(super::ListNode{
                    val: 2,
                    next: Some(Box::new(super::ListNode{
                        val: 4,
                        next: Some(Box::new(super::ListNode{
                            val: 3,
                            next: None,
                        })),
                    })),
                })),
                Some(Box::new(super::ListNode{
                    val: 5,
                    next: Some(Box::new(super::ListNode{
                        val: 6,
                        next: Some(Box::new(super::ListNode{
                            val: 4,
                            next: None,
                        })),
                    })),
                })),
                Some(Box::new(super::ListNode{
                    val: 7,
                    next: Some(Box::new(super::ListNode{
                        val: 0,
                        next: Some(Box::new(super::ListNode{
                            val: 8,
                            next: None,
                        })),
                    })),
                })),
            )
        ];

        for (l1, l2, expected) in examples {
            let result = super::Solution::add_two_numbers(l1, l2);

            assert_eq!(result, expected);
        }
    }
}
