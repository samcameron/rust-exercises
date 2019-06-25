use std::cmp::max;

pub fn find_largest_element1(elems: &Vec<i64>) -> Option<i64> {

    let mut largest: Option<i64> = None;

    for e in elems {
        match largest {
            Some(n) => largest = Some(max(n, *e)),
            None    => largest = Some(*e),
        }
    }

    largest
}

pub fn find_largest_element(elems: &Vec<i64>) -> Option<i64> {

    let first = elems.first()?;

    Some(find_largest_with_default(elems, * first))
}

fn find_largest_with_default(elems: &Vec<i64>, default: i64) -> i64 {
    let mut largest = default;
    for e in elems {
        largest = max(*e, largest);

    }
    largest
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_elements() {
        let empty_vec = Vec::new();

        let result = find_largest_element(&empty_vec);

        assert_eq!(result, None);
    }

    #[test]
    fn test_some_elements() {
        let myvec = vec![1, 5, 3, 8, 0];

        let result = find_largest_element(&myvec);

        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_negative_elements() {
        let myvec = vec![-1, -5, -3, -8];

        let result = find_largest_element(&myvec);

        assert_eq!(result, Some(-1));
    }
}
