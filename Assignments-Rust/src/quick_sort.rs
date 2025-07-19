/*  CSE 381 - Quick Sort
 *  (c) BYU-Idaho - It is an honor code violation to post this
 *  file completed in a public file sharing site. F5.
 *
*  Instructions: Refer to W04 Prove: Assignment in Canvas for detailed instructions.
 */

/* Use Quick Sort to sort a list of values in place
*
*  Inputs:
*     data - list of values
*  Outputs:
*     none
*/
 pub fn sort<T: Ord>(data : &mut [T]) {
    // We have to check for an empty 
    // list becuase len() returns a usize and
    // len-1 would be invalid.
    if data.is_empty() {
        return;
    }
    // Start the recursion and return whatever
    // result comes back.
    _sort(data, 0, data.len()-1)
}

/* Recursively use quick sort to sort a sublist
*  defined by first and last.
*
*  Inputs:
*     data - list of values
*     first - the starting index of the sublist
*     last - the ending inde of the sublist
*  Outputs:
*     None
*/
fn _sort<T: Ord>(data : &mut [T], first : usize, last : usize) {    
    todo!()
}

/* Partition a sublist by finding where a pivot belongs when sorted.  All
*  values less or equal to the pivot must be on the left hand side and
*  all values greater must be on the right hand size of the pivot.
*  In this implementation, do not select a random pivot.  Select the
*  last value in the sublist to always be your pivot.
*
*  Inputs:
*     data - list of values
*     first - the starting index of the sublist
*     last - the ending index of the sublist
*  Outputs:
*     The index of where the pivot was moved
*/
fn partition<T: Ord>(data : &mut [T], first : usize, last : usize) -> usize {
    todo!()
}

/* Do not modify these tests.
*  To run the tests, use the cargo tool from the command line:
*     Run All Tests: cargo test quick_sort
*     Run One Test:  cargo test quick_sort::tests::<test function name>
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_partition_position_middle() {
        let mut data = vec![3, 8, 1, 9, 2, 0, 5, 7, 6, 4];
        let result = partition(&mut data, 0, 9);
        assert_eq!(data, vec![3, 1, 2, 0, 4, 9, 5, 7, 6, 8]);
        assert_eq!(result, 4);
    }

    #[test]
    fn test2_partition_position_start() {
        let mut data = vec![3, 8, 1, 4, 2, 9, 5, 7, 6, 0];
        let result = partition(&mut data, 0, 9);
        assert_eq!(data, vec![0, 8, 1, 4, 2, 9, 5, 7, 6, 3]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test3_partition_position_end() {
        let mut data = vec![3, 8, 1, 4, 2, 0, 5, 7, 6, 9];
        let result = partition(&mut data,0, 9);
        assert_eq!(data, vec![3, 8, 1, 4, 2, 0, 5, 7, 6, 9]);
        assert_eq!(result, 9);
    }

    #[test]
    fn test4_sort() {
        let mut data = vec![3, 5, 2, 6, 1, 4];
        sort(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test5_already_sorted() {
        let mut data = vec![1, 2, 3, 4, 5, 6];
        sort(&mut data);
        assert_eq!(data, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test6_empty_sort() {
        let mut data: Vec<i32> = vec![];
        sort(&mut data);
        assert_eq!(data.len(), 0);
    }




}
