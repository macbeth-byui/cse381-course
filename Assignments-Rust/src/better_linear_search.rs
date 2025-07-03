/* CSE 381 - BetterLinearSerach
*  (c) BYU-Idaho - It is an honor code violation to post this
*  file completed in a public file sharing site. F5.
*
*  Instructions: Refer to W01 Prove: Assignment in Canvas for detailed instructions.
*/

#[allow(clippy::manual_find)]

/* Search for an item in a list.  Ignore duplicates by exiting
*  as soon as the first match is found.
*
*  Inputs:
*     data - list to search containing data that implements Eq (equality)
*     target - value to search for
*  Outputs:
*     An Option containing the index where the target was found: Some(index)
*
*  Errors: If the target is not found, then return the error Option: None
*/
pub fn search<T: Eq>(data : &[T], target : &T) -> Option<usize> {
    todo!()
} 

/* Do not modify these tests.
*  To run the tests, use the cargo tool from the command line:
*     Run All Tests: cargo test better_linear_search
*     Run One Test:  cargo test better_linear_search::tests::<test function name>
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_match_number() {
        let data = [1, 2, 3, 4, 5, 6];
        let target = 4;
        assert_eq!(search(&data, &target), Some(3));
    }

    #[test]
    fn test2_no_match_number() {
        let data = [1, 2, 3, 4, 5, 6];
        let target = -1;
        assert_eq!(search(&data, &target), None);
    }

    #[test]
    fn test3_match_string() {
        let data = ["cat", "dog", "pig", "chicken", "duck"];
        let target = "pig";
        assert_eq!(search(&data, &target), Some(2));
    }

    #[test]
    fn test4_no_match_string() {
        let data = ["cat", "dog", "pig", "chicken", "duck"];
        let target = "eagle";
        assert_eq!(search(&data, &target), None);
    }
	
	#[test]
    fn test5_empty() {
        let data = [];
        let target = 42;
        assert_eq!(search(&data, &target), None);
    }
	
	#[test]
    fn test6_match_first() {
        let data = [1, 2, 3, 4, 5, 6];
        let target = 1;
        assert_eq!(search(&data, &target), Some(0));
    }
	
	#[test]
    fn test7_match_last() {
        let data = [1, 2, 3, 4, 5, 6];
        let target = 6;
        assert_eq!(search(&data, &target), Some(5));
    }
	
}