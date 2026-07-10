/* CSE 381 - Huffman Tree
*  (c) BYU-Idaho - It is an honor code violation to post this
*  file completed in a public file sharing site.  F6.
*
*  Instructions: Refer to W10 Prove: Assignment in Canvas for detailed instructions.
*/

// NOTE: These statements will supress warnings in your code.  It is recommended
// that you comment these out when you start on this assignment
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::collections::HashMap;
use crate::pqueue::PQueue;

/* The Huffman Tree dynamically is organized a little differently in Rust.  In 
 * a form of polymorphism, we define each Node of the tree as being is a Support
 * node or a Leaf node.  A support node contains children and a leaf node contains
 * the encoded letter.  The tree is defined as the root node (either a leaf or a support
 * node) and the count that represents the tree.  When we create a tree, we will set 
 * the count in the Tree object and then create a root node (starting with a 
 * leaf node) to store in the Tree.
 * 
 * In the tree, the nodes will all be allocated on the Heap.  This is done
 * in Rust by using the Box<T> structure.  The Tree implements
 * the Eq, PartialEq, Hash, and Clone so that it will support the PQueue.
*/
#[derive(Debug, PartialEq, Hash, Clone)]
pub struct Tree {
    count : u32,
    root : Box<Node>,
}

impl Eq for Tree {
}

#[derive(Debug, PartialEq, Hash, Clone)]
enum Node {
    Support(Box<Node>, Box<Node>),
    Leaf(char)
}

/* Create a profile showing the frequency of all letters
*  from a string of text.
*
*  Inputs:
*     text - Source for the profile
*  Outputs:
*     HashMap mapping letters to frequencies
*/
pub fn profile(text : &str) -> HashMap<char, u32> {
   todo!() 
}

/* Create a huffman tree for all letters in the profile.  Use
*  a PQueue object (code already provided) in your implementation.
*
*  Inputs:
*     profile - Previously generated profile list of (letter,count) pairs
*  Outputs:
*     An Option containing a Tree object (which contains the root
*     node for the Huffman Tree): Some(tree)
*  Errors:
*     If the profile is empty, then return the error Option: None
*/
pub fn build_tree(profile : &HashMap<char, u32>) -> Option<Tree> {
    todo!()
}

/* Create an encoding map from the huffman tree
*
*  Inputs:
*     tree - An Option containing a Tree object (which includes
*            the root of the Huffman Tree): Some(tree)
*  Outputs:
*     A dictionary where key is the letter and value is the
*     huffman code.  The Tree object is Option None then 
*     an empty dictionary should be returned.
*/
pub fn create_encoding_map(tree : &Option<Tree>) -> HashMap<char,String> {
    let mut result = HashMap::new();

    if let Some(tree) = tree {
        _create_encoding_map(&tree.root, String::new(), &mut result);
    }

    result
}

/* Recursively visit each node in the Huffman Tree
*  looking for leaf nodes which contain letters.  Keep
*  track of the huffman code by adding 0 when going left
*  and 1 when going right.  If the tree has only one node
*  (which can be determined by node being a leaf but the
*  bit string is currently empty), then the one letter in 
*  the tree should be encoded as "1".
*
*  Inputs:
*     node - Current node we are on
*     code - Current bit string code created
*     map - Encoding Map being populated
*  Outputs:
*     none
*/
fn _create_encoding_map(node : &Node, bit_string : String, mapping : &mut HashMap<char,String>) {
    todo!()
}

/* Encode a string with the encoding map.
*
*  Inputs:
*     text - String to encode
*     map - Encoding Map previously created
*  Outputs:
*     An Option containing a string of huffman codes (1's and 0's) representing the
*     encoding of the text: Some(encoded)
*  Errors:
*     If a letter in the text does not exist in the encoding map, then the 
*     Option None should be returned.
*/
pub fn encode(text : &str, encoding : &HashMap<char,String>) -> Option<String> {
    todo!()
}

/* Decode a string with the huffman tree
*
*  Inputs:
*     text - String to decode
*     tree - An Option previously created containing a Tree object (which includes
*            the root of the Huffman Tree)
*  Outputs:
*     An Option containing the decoded text: Some(decoded)
*  Errors:
*     If there is an error decoding (bits don't match a letter) then 
*     an Option None should be returned.
*/
pub fn decode(bit_string : &str, tree : &Option<Tree>) -> Option<String> {
    todo!()
}

/* Do not modify these tests.
*  To run the tests, use the cargo tool from the command line:
*     Run All Tests: cargo test huffman_tree
*     Run One Test:  cargo test huffman_tree::tests::<test function name>
*  You can also use the "Run Test" or "Debug" links in Visual Studio Code 
*  next to each test function.
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_profile() {
        let text = "the rain in spain stays mainly in the plain";
        let profiled_text = profile(text);
        let mut expected_profile = HashMap::<char, u32>::new();
        expected_profile.insert(' ', 8);
        expected_profile.insert('a', 5);
        expected_profile.insert('e', 2);
        expected_profile.insert('h', 2);
        expected_profile.insert('i', 6);
        expected_profile.insert('l', 2);
        expected_profile.insert('m', 1);
        expected_profile.insert('n', 6);
        expected_profile.insert('p', 2);
        expected_profile.insert('r', 1);
        expected_profile.insert('s', 3);
        expected_profile.insert('t', 3);
        expected_profile.insert('y', 2);
        assert_eq!(profiled_text, expected_profile);
    }

    #[test]
    fn test2_build_tree() {
        let text = "the rain in spain stays mainly in the plain";
        let profiled_text = profile(text);
        let tree = build_tree(&profiled_text);
        assert!(matches!(tree, Some(t) if t.count == 43));
     }


    #[test]
    fn test3_create_encoding_map() {
        let tree = Some(Tree {
            count: 43,
            root: Box::new(Node::Support(
                Box::new(Node::Support(
                    Box::new(Node::Support(
                        Box::new(Node::Support(
                            Box::new(Node::Leaf('l')),
                            Box::new(Node::Leaf('e')),
                        )),
                        Box::new(Node::Support(
                            Box::new(Node::Leaf('h')),
                            Box::new(Node::Support(
                                Box::new(Node::Leaf('r')),
                                Box::new(Node::Leaf('m')),
                            )),
                        )),
                    )),
                    Box::new(Node::Support(
                        Box::new(Node::Support(
                            Box::new(Node::Leaf('p')),
                            Box::new(Node::Leaf('y')),
                        )),
                        Box::new(Node::Leaf('a')),
                    )),
                )),
                Box::new(Node::Support(
                    Box::new(Node::Support(
                        Box::new(Node::Support(
                            Box::new(Node::Leaf('s')),
                            Box::new(Node::Leaf('t')),
                        )),
                        Box::new(Node::Leaf('n')),
                    )),
                    Box::new(Node::Support(
                        Box::new(Node::Leaf('i')),
                        Box::new(Node::Leaf(' ')),
                    )),
                )),
            )),
        });
        let encoding = create_encoding_map(&tree);
        let mut expected = HashMap::<char, String>::new();
        expected.insert('l',"0000".to_string());
        expected.insert('e',"0001".to_string());
        expected.insert('h',"0010".to_string());
        expected.insert('r',"00110".to_string());
        expected.insert('m',"00111".to_string());
        expected.insert('p',"0100".to_string());
        expected.insert('y',"0101".to_string());
        expected.insert('a',"011".to_string());
        expected.insert('s',"1000".to_string());
        expected.insert('t',"1001".to_string());
        expected.insert('n',"101".to_string());
        expected.insert('i',"110".to_string());
        expected.insert(' ',"111".to_string());
        assert_eq!(encoding, expected);
    }

    #[test]
    fn test4_encode() {
        let text = "the rain in spain stays mainly in the plain";
        let mut encoding = HashMap::<char, String>::new();
        encoding.insert('l',"0000".to_string());
        encoding.insert('e',"0001".to_string());
        encoding.insert('h',"0010".to_string());
        encoding.insert('r',"00110".to_string());
        encoding.insert('m',"00111".to_string());
        encoding.insert('p',"0100".to_string());
        encoding.insert('y',"0101".to_string());
        encoding.insert('a',"011".to_string());
        encoding.insert('s',"1000".to_string());
        encoding.insert('t',"1001".to_string());
        encoding.insert('n',"101".to_string());
        encoding.insert('i',"110".to_string());
        encoding.insert(' ',"111".to_string());
        let encoded_bits = encode(text, &encoding);
        assert!(encoded_bits.is_some());

        assert_eq!(encoded_bits.unwrap(), "10010010000111100110011110101111110101111100001000111101011111000100101101011000111001110111101010000010111111010111110010010000111101000000011110101");
    }

    #[test]
    fn test5_decode() {
        let text = "the rain in spain stays mainly in the plain";
        let profiled_text = profile(text);
        let tree = build_tree(&profiled_text);
        let encoding = create_encoding_map(&tree);
        let encoded_bits = encode(text, &encoding);
        assert!(encoded_bits.is_some());
        let decoded_text = decode(&encoded_bits.unwrap(), &tree);
        assert!(decoded_text.is_some());
        assert_eq!(text, decoded_text.unwrap());
    }


}


