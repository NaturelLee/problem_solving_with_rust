use super::stack::Stack;

// 判断()是否匹配
fn pairs_checker_one(pair: &str) -> bool {
    let mut stack = Stack::new();
    for c in pair.chars() {
        if c == '(' {
            stack.push(c);
        } else if c == ')' {
            if stack.is_empty() {
                return false;
            }
            stack.pop();
        }
    }
    stack.is_empty()
}

const LEFT: &'static str = "([{";
const RIGHT: &'static str = ")]}";

fn get_index(s: char, side: &str) -> Option<usize> {
    // for (i, c) in side.chars().enumerate() {
    //     if c == s {
    //         return Some(i);
    //     }
    // }
    // None
    side.find(s)
}

// 判断[](){}是否匹配
fn pairs_checker_two(pair: &str) -> bool {
    let mut stack = Stack::new();
    for c in pair.chars() {
        if LEFT.contains(c) {
            stack.push(c);
        } else if RIGHT.contains(c) {
            if stack.is_empty() {
                return false;
            }
            // 获取右括号的索引
            let right_pair_index = get_index(c, RIGHT);
            // 取出最近的左括号
            let left_pair = stack.pop();
            // 获取左括号的索引
            let left_pair_index = get_index(left_pair.unwrap(), LEFT);

            if right_pair_index != left_pair_index {
                return false;
            }
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_pairs_checker_one() {
        assert!(pairs_checker_one("(a + b) * (c + d)"));
        assert!(pairs_checker_one("(())"));
        assert!(!pairs_checker_one(")("));
        assert!(!pairs_checker_one("("));
        assert!(!pairs_checker_one(")"));
    }

    #[test]
    fn test_pairs_checker_two() {
        assert!(pairs_checker_two(
            "{[(a + b) * (c - d)]}[(a * a)*(b * b)]()"
        ));
        assert!(pairs_checker_two("[()]"));
        assert!(!pairs_checker_two("]["));
        assert!(!pairs_checker_two("["));
        assert!(!pairs_checker_two("]"));
    }
}
