# LeetCode Solutions in Rust

My solutions to LeetCode problems, written in Rust.

## Structure

Solutions are organized by topic. Each topic is a module under `src/`, and each problem is a separate file within its topic.

```
src/
├── lib.rs
├── array/
│   ├── mod.rs
│   ├── two_sum.rs
│   └── ...
├── linked_list/
│   ├── mod.rs
│   └── add_two_numbers.rs
├── string/
├── tree/
├── graph/
├── dynamic_programming/
├── binary/
│   ├── mod.rs
│   └── binary_search.rs
└── ...
```

## Progress: Blind 75

`▓░░░░░░░░░░░░░░░░░░░ 4/75 (5%)`

### Array

- [x] [Two Sum](https://leetcode.com/problems/two-sum)
- [ ] Best Time to Buy and Sell Stock
- [x] [Contains Duplicate](https://leetcode.com/problems/contains-duplicate)
- [ ] Product of Array Except Self
- [ ] Maximum Subarray
- [ ] Maximum Product Subarray
- [x] [Find Minimum in Rotated Sorted Array](https://leetcode.com/problems/find-minimum-in-rotated-sorted-array)
- [ ] Search in Rotated Sorted Array
- [ ] 3Sum
- [ ] Container With Most Water

### Binary

- [ ] Sum of Two Integers
- [ ] Number of 1 Bits
- [ ] Counting Bits
- [ ] Missing Number
- [ ] Reverse Bits

### Dynamic Programming

- [ ] Climbing Stairs
- [ ] Coin Change
- [ ] Longest Increasing Subsequence
- [ ] Longest Common Subsequence
- [ ] Word Break
- [ ] Combination Sum IV
- [ ] House Robber
- [ ] House Robber II
- [ ] Decode Ways
- [ ] Unique Paths
- [ ] Jump Game

### Graph

- [ ] Clone Graph
- [ ] Course Schedule
- [ ] Pacific Atlantic Water Flow
- [ ] Number of Islands
- [ ] Longest Consecutive Sequence
- [ ] Alien Dictionary
- [ ] Graph Valid Tree
- [ ] Number of Connected Components in an Undirected Graph

### Interval

- [ ] Insert Interval
- [ ] Merge Intervals
- [ ] Non-overlapping Intervals
- [ ] Meeting Rooms
- [ ] Meeting Rooms II

### Linked List

- [ ] Reverse a Linked List
- [ ] Detect Cycle in a Linked List
- [ ] Merge Two Sorted Lists
- [ ] Merge K Sorted Lists
- [ ] Remove Nth Node From End of List
- [ ] Reorder List

### Matrix

- [ ] Set Matrix Zeroes
- [ ] Spiral Matrix
- [ ] Rotate Image
- [ ] Word Search

### String

- [ ] Longest Substring Without Repeating Characters
- [ ] Longest Repeating Character Replacement
- [ ] Minimum Window Substring
- [x] [Valid Anagram](https://leetcode.com/problems/valid-anagram)
- [ ] Group Anagrams
- [ ] Valid Parentheses
- [ ] Valid Palindrome
- [ ] Longest Palindromic Substring
- [ ] Palindromic Substrings
- [ ] Encode and Decode Strings

### Tree

- [ ] Maximum Depth of Binary Tree
- [ ] Same Tree
- [ ] Invert Binary Tree
- [ ] Binary Tree Maximum Path Sum
- [ ] Binary Tree Level Order Traversal
- [ ] Serialize and Deserialize Binary Tree
- [ ] Subtree of Another Tree
- [ ] Construct Binary Tree from Preorder and Inorder Traversal
- [ ] Validate Binary Search Tree
- [ ] Kth Smallest Element in a BST
- [ ] Lowest Common Ancestor of a BST
- [ ] Implement Trie (Prefix Tree)
- [ ] Add and Search Word
- [ ] Word Search II

### Heap

- [ ] Merge K Sorted Lists
- [ ] Top K Frequent Elements
- [ ] Find Median from Data Stream

## Extras (not Blind 75)

Solutions outside the Blind 75 list — classic algorithms, warm-ups, or topic detours.

### Binary Search

- [x] Binary Search (recursive) — `src/binary/binary_search.rs`
- [x] Binary Search (iterative) — `src/binary/binary_search.rs`
- [x] Find the First True in a Sorted Boolean Array — `src/binary/first_true_in_sorted_boolean_array.rs`
- [x] First Element Not Smaller Than Target — `src/binary/first_element_not_smaller_than_target.rs`
- [x] Find Element in Sorted Array with Duplicates — `src/binary/find_element_in_sorted_array_with_duplicates.rs`
- [x] Square Root Estimation — `src/binary/square_root_estimation.rs`
- [x] [Peak Index in a Mountain Array](https://leetcode.com/problems/peak-index-in-a-mountain-array) — `src/binary/peak_index_in_mountain_array.rs`

## Usage

Run all tests:

```sh
cargo test
```

Run tests for a specific topic:

```sh
cargo test array
```

Run tests for a specific problem:

```sh
cargo test two_sum
```
