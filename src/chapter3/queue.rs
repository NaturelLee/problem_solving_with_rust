#[derive(Debug)]
struct Queue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    fn new(size: usize) -> Self {
        Queue {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn enqueue(&mut self, item: T) -> Result<(), String> {
        if self.size() < self.cap {
            self.data.insert(0, item);
            return Ok(());
        }

        Err("No space left".to_string())
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.data.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_queue() {
        let mut q = Queue::new(4);
        assert!(q.is_empty());
        assert_eq!(q.size(), 0);

        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);

        assert!(!q.is_empty());
        assert_eq!(q.size(), 3);

        let first = q.dequeue();
        assert!(!q.is_empty());
        assert_eq!(q.size(), 2);
        assert_eq!(first.unwrap(), 1);
        q.dequeue();
        q.dequeue();
        assert!(q.is_empty());
        assert_eq!(q.size(), 0);
    }
}
