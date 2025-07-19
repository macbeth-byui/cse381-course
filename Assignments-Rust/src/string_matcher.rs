/* CSE 381 - String Matcher
*  (c) BYU-Idaho - It is an honor code violation to post this
*  file completed in a public file sharing site. F5.
*
*  Instructions: Refer to W08 Prove: Assignment in Canvas for detailed instructions.
*/

use std::collections::HashMap;

/* Find all matches of the pattern in the string of text given a list
*  of all valid input characters.  This function needs to build the Finite
*  State Machine table by calling build_fsm.
*
*  Inputs:
*     text - string to search for pattern
*     pattern - substring to search in the text
*     inputs - valid characters using in the text and pattern
*  Outputs:
*     An Option containing the list of indices where the pattern matched (last char of pattern match)
*  Errors:
*     If a lookup in the FSM table fails, then return an error Option: None.
*/
pub fn match_pattern(text : &str, pattern : &str, inputs : &[char]) -> Option<Vec<usize>> {
   todo!()
}

/* Build the Finite State Machine table for the pattern and list of valid
*  inputs provided.
*
*  Inputs:
*     pattern - string to match
*     inputs - valid list of characters that could be seen
*  Outputs:
*     Finite State Machine defined by a list of dictionaries.  Each index
*     in the list represents a state in the FSM (index 0 is first).  The
*     dictionary shows the next state to goto for each of the valid
*     inputs that can occur.
*/
pub fn build_fsm(pattern : &str, inputs : &[char]) -> Vec<HashMap<char, usize>> {
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
    fn test1_build_fsm1() {
        let fsm = build_fsm("AAC", &['A', 'C', 'G', 'T']);
        assert_eq!(fsm, vec![
            vec![('A', 1), ('C', 0), ('G', 0), ('T', 0)].into_iter().collect(),
            vec![('A', 2), ('C', 0), ('G', 0), ('T', 0)].into_iter().collect(),
            vec![('A', 2), ('C', 3), ('G', 0), ('T', 0)].into_iter().collect(),
            vec![('A', 1), ('C', 0), ('G', 0), ('T', 0)].into_iter().collect(),
        ]);
    }

    #[test]
    fn test2_build_fsm1() {
        let fsm = build_fsm("CBCBA", &['A', 'B', 'C']);
        assert_eq!(fsm, vec![
            vec![('A', 0), ('B', 0), ('C', 1)].into_iter().collect(),
            vec![('A', 0), ('B', 2), ('C', 1)].into_iter().collect(),
            vec![('A', 0), ('B', 0), ('C', 3)].into_iter().collect(),
            vec![('A', 0), ('B', 4), ('C', 1)].into_iter().collect(),        
            vec![('A', 5), ('B', 0), ('C', 3)].into_iter().collect(),        
            vec![('A', 0), ('B', 0), ('C', 1)].into_iter().collect(),        
        ]);
    }

    #[test]
    fn test3_match1() {
        let results = match_pattern("GTAACAGTAAACG", "AAC", &['A', 'C', 'G', 'T']);
        assert!(results.is_some());
        assert_eq!(results.unwrap(), vec![4, 11]);
    }

    #[test]
    fn test4_match2() {
        let results = match_pattern("GTAACAGTAAACG", "AA", &['A', 'C', 'G', 'T']);
        assert!(results.is_some());
        assert_eq!(results.unwrap(), vec![3, 9, 10]);
    }

    #[test]
    fn test5_match3() {
        let results = match_pattern("ABCBCABCBCBC", "CBC", &['A', 'B', 'C']);
        assert!(results.is_some());
        assert_eq!(results.unwrap(), vec![4, 9, 11]);
    }

    #[test]
    fn test6_no_matches() {
        let results = match_pattern("GTAACAGTAAACG", "AACT", &['A', 'C', 'G', 'T']);
        assert!(results.is_some());
        assert_eq!(results.unwrap(), vec![]);
    }
}