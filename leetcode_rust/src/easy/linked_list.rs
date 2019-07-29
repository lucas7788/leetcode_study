
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val :i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode{
            next:None,
            val
        }
    }

    pub fn add(&mut self, item: ListNode) {
        match &mut self.next {
            Some(next) => {
                next.add(item);
            }
            None => {
                self.next = Some(Box::new(item));
            }
        }
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    add_two_numbers2(l1, l2, 0)
}

pub fn add_two_numbers2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut jinwei: i32) -> Option<Box<ListNode>> {
    if l1 == None && l2 == None && jinwei == 0{
        return None
    }
    let mut a = 0;
    let mut b = 0;
    let mut l1a = Box::new(ListNode::new(0));
    let mut l2a = Box::new(ListNode::new(0));
    match l1 {
        Some(mut l1b) => {
            l1a = l1b;
            a = l1a.val;
        }
        None => {
            a = 0;
        }
    }
    match l2 {
        Some(mut l2b) => {
            l2a = l2b;
            b = l2a.val;
        }
        None => {
            b = 0;
        }
    }
    let mut v = a+b+jinwei;
    if v >= 10 {
        v -= 10;
        jinwei = 1;
    } else {
        jinwei = 0;
    }
    let mut res = ListNode::new(v);
    let t = add_two_numbers2(l1a.next, l2a.next, jinwei);
    match t {
        Some(node) => {
            res.add(*node);
            return Some(Box::new(res));
        }
        None => {
            return Some(Box::new(res));
        }
    }
}



fn linked_node_test() {
    let mut l1 = ListNode::new(2);
    l1.add(ListNode::new(4));
    l1.add(ListNode::new(3));

    println!("l1: {:?}", l1);

    let mut l2 = ListNode::new(5);
    l2.add(ListNode::new(6));
    l2.add(ListNode::new(4));

    println!("l2: {:?}", l2);

    let res = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
    match res {
        Some(r) => {
            println!("res: {:?}", r);
        }
        None => {
            println!("None");
        }
    }

//    let mut res = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));

//    println!("res: {}", res.unwrap().val);
}