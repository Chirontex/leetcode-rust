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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result: Vec<i32> = vec![];
        let mut additional: i32 = 0;
        let mut n1 = &l1;
        let mut n2 = &l2;

        loop {
            let v: i32;
            let v1 = Self::get_val_from_node(n1);
            let v2 = Self::get_val_from_node(n2);

            if (v1 == 0 && n1.is_none()) && (v2 == 0 && n2.is_none()) && additional == 0 {
                break;
            }

            v = v1 + v2 + additional;

            if v >= 10 {
                additional = 1;
                result.push(v - 10);
            } else {
                additional = 0;
                result.push(v);
            }

            n1 = Self::get_next_node(n1);
            n2 = Self::get_next_node(n2);
        }

        if result.is_empty() {
            result.push(0);
        }

        result.reverse();

        Self::vec_to_reverse_list(result)
    }

    fn get_val_from_node(node: &Option<Box<ListNode>>) -> i32 {
        match node {
            Some(n) => n.val,
            None => 0,
        }
    }

    fn get_next_node(node: &Option<Box<ListNode>>) -> &Option<Box<ListNode>> {
        match node {
            Some(n) => &n.next,
            None => &None,
        }
    }

    fn vec_to_reverse_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
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
                super::Solution::vec_to_reverse_list(l1),
                super::Solution::vec_to_reverse_list(l2),
            );

            assert_eq!(result, super::Solution::vec_to_reverse_list(expected));
        }
    }
}
