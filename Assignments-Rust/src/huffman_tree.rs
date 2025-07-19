/* CSE 381 - Huffman Tree
*  (c) BYU-Idaho - It is an honor code violation to post this
*  file completed in a public file sharing site. F5.
*
*  Instructions: Refer to W10 Prove: Assignment in Canvas for detailed instructions.
*/

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
*     List of (letter,count) pairs that represent the profile
*     of the text.  This list must be sorted by letter to ensure
*     consistent huffman tree creation.
*/
pub fn profile(text : &str) -> Vec<(char, u32)> {
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
pub fn build_tree(profile : &[(char, u32)]) -> Option<Tree> {
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
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_profile() {
        let text = "the rain in spain stays mainly in the plain";
        let profiled_text = profile(text);
        let expected_profile = vec![
            (' ', 8),
            ('a', 5),
            ('e', 2),
            ('h', 2),
            ('i', 6),
            ('l', 2),
            ('m', 1),
            ('n', 6),
            ('p', 2),
            ('r', 1),
            ('s', 3),
            ('t', 3),
            ('y', 2),
        ];
        assert_eq!(profiled_text, expected_profile);
    }

    #[test]
    fn test2_build_tree() {
        let text = "aabbbccccdde";
        let profiled_text = profile(text);
        let tree = build_tree(&profiled_text);
        assert!(tree.is_some());
        let expected_tree = Tree {count: 12, root :
            Box::new(Node::Support(
                Box::new(Node::Support(
                    Box::new(Node::Leaf('a')),
                    Box::new(Node::Support(
                        Box::new(Node::Leaf('e')),
                        Box::new(Node::Leaf('d')))
                    ),
                )),
                Box::new(Node::Support(
                    Box::new(Node::Leaf('b')),
                    Box::new(Node::Leaf('c'))
                )),
            )) 
        };
        assert_eq!(tree.unwrap(), expected_tree);
        
     }


    #[test]
    fn test3_create_encoding_map() {
        let text = "the rain in spain stays mainly in the plain";
        let profiled_text = profile(text);
        let tree = build_tree(&profiled_text);
        let encoding = create_encoding_map(&tree);
        let mut expected = HashMap::<char, String>::new();
        expected.insert('l',"11101".to_string());
        expected.insert('e',"11111".to_string());
        expected.insert('h',"11110".to_string());
        expected.insert('r',"01011".to_string());
        expected.insert('m',"01010".to_string());
        expected.insert('p',"0100".to_string());
        expected.insert('y',"11100".to_string());
        expected.insert('a',"011".to_string());
        expected.insert('s',"1001".to_string());
        expected.insert('t',"1000".to_string());
        expected.insert('n',"101".to_string());
        expected.insert('i',"110".to_string());
        expected.insert(' ',"00".to_string());
        assert_eq!(encoding, expected);
    }

    #[test]
    fn test4_encode() {
        let text = "the rain in spain stays mainly in the plain";
        let profiled_text = profile(text);
        let tree = build_tree(&profiled_text);
        let encoding = create_encoding_map(&tree);
        let encoded_bits = encode(text, &encoding);
        assert!(encoded_bits.is_some());

        assert_eq!(encoded_bits.unwrap(), "10001111011111000101101111010100110101001001010001111010100100110000111110010010001010011110101111011110000110101001000111101111100010011101011110101");
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


