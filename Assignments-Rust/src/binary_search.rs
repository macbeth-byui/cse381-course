/* CSE 381 - Binary Search
*  (c) BYU-Idaho - It is an honor code violation to post this
*  file completed in a public file sharing site. F5.
*
*  Instructions: Refer to W02 Prove: Assignment in Canvas for detailed instructions.
*/

use std::cmp::Ordering;

/* Use Binary Search to search for an item in a list.
* 
*  Inputs:
*     data - list to search containing data that implements Ord (ordered)
*     target - value to search for
*  Outputs:
*     An Option containing the index where the target was found: Some(index)
*
*  Errors: If the target is not found, then return the error Option: None
*/
pub fn search<T: Ord>(data : &[T], target : &T) -> Option<usize> {
    // We have to check for an empty 
    // list becuase len() returns a usize and
    // len-1 would be invalid.
    if data.is_empty() {
        return None;
    }
    // Start the recursion and return whatever
    // results comes back.
    _search(data, target, 0, data.len()-1)
}

/* Use Binary Search to recursively search for an item in a sublist.
*
*  Inputs:
*     data - list to search
*     target - value to search for
*     first - starting index of sublist of data
*     last - ending index of sublist of data
*  Outputs:
*     An Option containing the index where the target was found: Some(index)
*
*  Errors: If the target is not found, then return the error Option: None
*/
fn _search<T: Ord>(data : &[T], target : &T, first : usize, last : usize) -> Option<usize> {
    todo!()
}

/* Do not modify these tests.
*  To run the tests, use the cargo tool from the command line:
*     Run All Tests: cargo test binary_search
*     Run One Test:  cargo test binary_search::tests::<test function name>
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_match_first() {
        let data = vec![1, 3, 6, 7, 11, 13, 15];
        let result = search(&data, &1);
        assert_eq!(result,Some(0));
    }
	
	#[test]
    fn test2_match_last() {
        let data = vec![1, 3, 6, 7, 11, 13, 15];
        let result = search(&data, &15);
        assert_eq!(result,Some(6));
    }
	
    #[test]
    fn test3_match_middle() {
        let data = vec![1, 3, 6, 7, 11, 13, 15];
        let result = search(&data, &7);
        assert_eq!(result,Some(3));
    }
	
    #[test]
    fn test4_no_match_too_big() {
        let data = vec![1, 3, 6, 7, 11, 13, 15];
        let result = search(&data, &20);
        assert_eq!(result,None);
    }
	
    #[test]
    fn test5_no_match_too_small() {
        let data = vec![1, 3, 6, 7, 11, 13, 15];
        let result = search(&data, &0);
        assert_eq!(result,None);
    }
	
    #[test]
    fn test6_no_match_middle() {
        let data = vec![1, 3, 6, 7, 11, 13, 15];
        let result = search(&data, &4);
        assert_eq!(result,None);
    }
	
    #[test]
    fn test7_empty() {
        let data = vec![];
        let result = search(&data, &7);
        assert_eq!(result,None);
    }
	
	
	

}