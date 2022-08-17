use super::stack::Stack;

fn base_converter(mut number: u64, base: u64) -> String {
    let digit = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];

    let mut stack = Stack::new();

    while number != 0 {
        let remain = number % base;
        stack.push(remain);
        number /= base;
    }

    let mut result = "".to_string();
    while !stack.is_empty() {
        let index = stack.pop().unwrap();
        result += &digit[index as usize].to_string();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_base_converter() {
        assert_eq!(base_converter(10, 2), "1010");
        assert_eq!(base_converter(11, 2), "1011");
        assert_eq!(base_converter(11, 8), "13");
        assert_eq!(base_converter(17, 16), "11");
        assert_eq!(base_converter(100000, 16), "186A0");
    }
}
