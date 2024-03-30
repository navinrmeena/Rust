// Function to check if a string is a palindrome
fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

// Function to find the index of the first occurrence of a given number in a sorted array
fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

// Function to return the shortest word in a string of words
fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|word| word.len())
}

// Function to check if a number is prime
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

// Function to find the median of a sorted array
fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        let mid = len / 2;
        (arr[mid - 1] + arr[mid]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

// Function to find the longest common prefix of a set of strings
fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let mut prefix = strs[0].clone();
    for s in strs.iter().skip(1) {
        prefix = prefix.chars().zip(s.chars())
            .take_while(|&(a, b)| a == b)
            .map(|(a, _)| a)
            .collect();
    }
    prefix
}

// Function to return the kth smallest element in an array
fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    arr.iter().cloned().nth(k - 1)
}

// Definition of a binary tree node
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

// Function to find the maximum depth of a binary tree
fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    // Testing the functions
    println!("Is 'racecar' a palindrome? {}", is_palindrome("racecar"));
    println!("Index of first occurrence of 3: {:?}", first_occurrence(&[1, 2, 3, 3, 4], 3));
    println!("Shortest word: {:?}", shortest_word("The quick brown fox"));
    println!("Is 17 prime? {}", is_prime(17));
    println!("Median: {}", median(&[1, 2, 3, 4, 5]));
    println!("Longest common prefix: {}", longest_common_prefix(&["flower".to_string(), "flow".to_string(), "flight".to_string()]));
    println!("3rd smallest element: {:?}", kth_smallest(&[1, 2, 3, 4, 5], 3));
    // Constructing a sample binary tree
    let root = TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode {
            val: 2,
            left: None,
            right: None,
        })),
        right: Some(Box::new(TreeNode {
            val: 3,
            left: None,
            right: Some(Box::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            })),
        })),
    };
    println!("Maximum depth of binary tree: {:?}", max_depth(Some(Box::new(root))));
}
