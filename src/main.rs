
#[derive(Debug)]
struct Queue {
    older: Vec<i32>,
    younger: Vec<i32>,
}

impl Queue {
    fn push(&mut self, value: i32) {
        self.younger.push(value);
    }

    fn pop(&mut self) -> Option<i32> {
        if self.older.is_empty() {
            None
        } else {
            self.older.pop()
        }
    }
}


fn main() {
    let mut queue = Queue { older: vec![], younger: vec![] };

    queue.push(32);
    queue.push(62);
    queue.push(52);
    queue.push(62);

    let old_person = queue.pop();

    dbg!(queue, old_person);
}
