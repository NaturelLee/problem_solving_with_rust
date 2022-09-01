#[derive(Debug)]
pub struct Deque<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Deque<T> {
    pub fn new(cap: usize) -> Self {
        Deque {
            cap,
            data: Vec::with_capacity(cap),
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    pub fn add_rear(&mut self, item: T) -> Result<(), String> {
        if self.size() == self.cap() {
            return Err("No space available".to_string());
        }
        self.data.push(item);
        Ok(())
    }

    pub fn add_front(&mut self, item: T) -> Result<(), String> {
        if self.size() == self.cap() {
            return Err("No space available".to_string());
        }
        self.data.insert(0, item);
        Ok(())
    }

    pub fn remove_rear(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.data.pop()
        }
    }

    pub fn remove_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deque() {
        let mut dq: Deque<usize> = Deque::new(10);
        assert_eq!(dq.cap, 10);
        assert!(dq.is_empty());
        dq.add_rear(1);
        dq.add_rear(2);
        dq.add_front(3);
        dq.add_front(4);
        assert_eq!(dq.size(), 4);
        assert!(!dq.is_empty());
        assert_eq!(dq.remove_rear(), Some(2));
        assert_eq!(dq.remove_front(), Some(4));
        assert_eq!(dq.size(), 2);
    }
}
