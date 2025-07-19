/* CSE 381 - Merge Sort
*  (c) BYU-Idaho - It is an honor code violation to post this
*  file completed in a public file sharing site. F5.
*
*  Instructions: Refer to W03 Prove: Assignment in Canvas for detailed instructions.
*/

/* Use Merge Sort to sort a list of values in place
*
*  Inputs:
*     data - list of values
*  Outputs:
*     none
*/
pub fn sort<T: Ord + Clone>(data : &mut [T]) {
    // We have to check for an empty 
    // list becuase len() returns a usize and
    // len-1 would be invalid.
    if data.is_empty() {
        return
    }
    // Start the recursion and return whatever
    // comes back.
    _sort(data, 0, data.len()-1)
}

/* Recursively use merge sort to sort a sublist
    * defined by first and last.
    * 
    *  Inputs:
    *     data - list of values
    *     first - the starting index of the sublist
    *     last - the ending index of the sublist
    *  Outputs:
    *     None
    */
fn _sort<T: Ord + Clone>(data : &mut [T], first : usize, last : usize) {
    todo!()
}

/* Merge two sorted list which are adjacent to each other back into
* the same list.
*
*  Inputs:
*     data - list of values
*     first - the starting index of the first sorted sublist
*     mid - the ending index of the first sorted sublist (second sublist starts after)
*     last - the ending index of the second sorted sublist
*  Outputs:
*     None
*/
fn merge<T: Ord + Clone>(data : &mut [T], first : usize, mid : usize, last : usize) {
    todo!()
}

/* Do not modify these tests.
*  To run the tests, use the cargo tool from the command line:
*     Run All Tests: cargo test merge_sort
*     Run One Test:  cargo test merge_sort::tests::<test function name>
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_merge_equal_sized_sublists() {
        let mut data = vec![1, 4, 6, 7, 8, 2, 3, 5, 9, 10];
        merge(&mut data, 1, 4, 8);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test2_merge_unequal_sized_sublists1() {
        let mut data = vec![1, 4, 6, 7, 8, 9, 2, 3, 5, 10];
        merge(&mut data, 1, 5, 8);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test3_merge_unequal_sized_sublists2() {
        let mut data = vec![1, 2, 4, 6, 3, 5, 7, 8, 9, 10];
        merge(&mut data, 1, 3, 8);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test4_merge_already_sorted() {
        let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        merge(&mut data, 1, 4, 8);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
    
    #[test]
    fn test5_merge_small_1() {
        let mut data = vec![1, 2, 3, 4, 6, 5, 7, 8, 9, 10];
        merge(&mut data, 4, 4, 6);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test6_merge_small_2() {
        let mut data = vec![1, 2, 3, 4, 6, 7, 5, 8, 9, 10];
        merge(&mut data, 4, 5, 6);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test7_sort_even() {
        let mut data = vec![3, 5, 2, 6, 1, 4];
        sort(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test8_sort_odd() {
        let mut data = vec![3, 5, 7, 2, 6, 1, 4];
        sort(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test9_already_sorted() {
        let mut data = vec![1, 2, 3, 4, 5, 6];
        sort(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test10_empty() {
        let mut data: Vec<i32> = vec![];
        sort(&mut data);
        assert_eq!(data, vec![]);
    }
    

}