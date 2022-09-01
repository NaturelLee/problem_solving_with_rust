// palindrome detection 回文检测
use super::deque::Deque;

fn is_even(num: usize) -> bool {
    num % 2 == 0
}

fn palindrome_detection(item: &str) -> bool {
    let str_len = item.len();
    if str_len == 1 {
        return true;
    }

    let mut dq: Deque<char> = Deque::new(str_len);
    item.chars().for_each(|ch| {
        dq.add_rear(ch);
    });

    let mut is_true = true;

    loop {
        let rear = dq.remove_rear();
        let front = dq.remove_front();

        if rear != front {
            is_true = false;
            break;
        }

        if dq.size() < 2 {
            if is_even(str_len) {
                if dq.is_empty() {
                    is_true = true;
                } else {
                    is_true = false;
                }
            } else {
                if dq.size() == 1 {
                    is_true = true;
                } else {
                    is_true = false;
                }
            }

            break;
        }
    }

    is_true
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_palindrome_detection() {
        assert!(palindrome_detection("q"));
        assert!(palindrome_detection("aqa"));
        assert!(!palindrome_detection("acqac"));
        assert!(palindrome_detection("abcqcba"));
        assert!(palindrome_detection("abccba"));
    }
}
