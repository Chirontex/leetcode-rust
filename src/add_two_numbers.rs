// https://leetcode.com/problems/add-two-numbers
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

#[allow(dead_code)]
impl Solution {
    // TODO: "1567 / 1569 testcases passed", so must keep working on it
    // TODO: move private methods to a trait implementation
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::decompile_number(Self::compile_number(&l1) + Self::compile_number(&l2))
    }

    fn compile_number(list: &Option<Box<ListNode>>) -> u128 {
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
                let num: u128 = num.try_into().unwrap();
                let rate = 10u128.pow((med.len() - i - 1) as u32);

                result += num * rate;
            }
        }

        result
    }

    fn decompile_number(number: u128) -> Option<Box<ListNode>> {
        let mut med: Vec<i32> = vec![];

        for num in number.to_string().chars() {
            med.push(String::from(num).parse().unwrap());
        }

        if med.len() > 0 {
            return Self::vec_to_list(med);
        }

        None
    }

    fn vec_to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut previous: Option<Box<ListNode>> = None;

        for &num in vec.iter() {
            previous = Some(Box::new(ListNode {
                val: num,
                next: previous,
            }));
        }

        previous
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add_two_numbers() {
        let examples = vec![
            (vec![2, 4, 3], vec![5, 6, 4], vec![7, 0, 8]),
            (
                vec![
                    2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4,
                    3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2,
                    4, 3, 2, 4, 3, 2, 4, 3, 9,
                ],
                vec![
                    5, 6, 4, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4,
                    3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2, 4, 3, 2,
                    4, 3, 2, 4, 3, 9, 9, 9, 9,
                ],
                vec![
                    7, 0, 8, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8,
                    6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4, 8, 6, 4,
                    8, 6, 4, 8, 6, 1, 4, 3, 9, 1,
                ],
            ),
        ];

        for (mut l1, mut l2, mut expected) in examples {
            l1.reverse();
            l2.reverse();
            expected.reverse();

            let result = super::Solution::add_two_numbers(
                super::Solution::vec_to_list(l1),
                super::Solution::vec_to_list(l2),
            );

            assert_eq!(result, super::Solution::vec_to_list(expected));
        }
    }
}
