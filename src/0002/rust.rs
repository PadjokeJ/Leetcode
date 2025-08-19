// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1.as_ref(), l2.as_ref());
        let mut l3 = Some(Box::new(ListNode { val: 0, next: None }));
        let mut head = l3.as_mut();

        let mut carry: i32 = 0;

        while l1.is_some() || l2.is_some() {
            let mut sum: i32 = carry;
            if let Some(n) = l1 {
                sum += n.val;
                l1 = n.next.as_ref();
            }
            if let Some(n) = l2 {
                sum += n.val;
                l2 = n.next.as_ref();
            }
            
            carry = sum / 10;

            head.as_mut().unwrap().next = Some(Box::new(ListNode { val: sum % 10, next: None }));
            head = head.unwrap().next.as_mut();
        }

        if carry == 1 {
            head.as_mut().unwrap().next = Some(Box::new(ListNode { val: 1, next: None }));
        }

        l3.unwrap().next
    }
}
