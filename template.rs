fn part(input: &str) -> String {
    assert!(!input.is_empty());
    input.into()
}

fn main() {
    let input = include_str!("./input.txt");
    let result = part(input);

    println!("result: {result}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let test_input = include_str!("./test.txt");
        let test_output = "placeholder";
        assert_eq!(part(test_input), test_output);
    }
}
