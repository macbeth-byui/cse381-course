# Algorithm Report

Author: Chad Macbeth

Date: 06/13/2023

## 1. Name
Better Linear Search

## 2. Overview & Purpose
The Better Linear Search will search an unsorted list of numbers until either the target is found or until all items in the list have been checked.

## 3. Methodology

The Better Linear Search is a basic search of unsorted data $\lang c_1, c_2, c_3, ... c_n \rang$ for a target $t$.  Since the data is unsorted, we need to start our search with $c_1$ and compare all values until a matching value is found.  When a matching value is found, the algorithm will immediately return the index of the matching value.  If the value is not found, then -1 will be returned.

![](images/BetterLinearSearch.drawio.png)

## 4. Analysis Results

The performance for the search (where $n$ is the number of items in the list):

* Worst Case: $O(n)$
* Best Case: $\Omega(1)$

## 5. AI Research

### 5.1 Research

Research this algorithm on ChatGPT by asking at least 4 questions (e.g. follow-up questions from the previous response, things from the reading or class that was confusing).  Identify at least 2 things that you learned about the algorithm from your research:

1. The technique of this algorithm can be applied in multiple ways including finding all the duplicates, the first instance, or the last instance.  When you traverse a linked list, you are using this algorithm.
2. It's better to sort if I am going to perform multiple searches.  If I need search very quickly I can create indicies by using hash tables.

### 5.2 Code Comparison

Ask ChatGPT to write code for this algorithm in C#.  Compare the code that you wrote with the code that ChatGPT responded with.  Identify any differences from your code and explain why they are different:

1. The function interface works with an array of integer while my code works with a List of IComparables.  This function can be modified to work with an array or list of any data type that supports comparison.  List of IComparable is more generic that an array of integers.

### 5.3 Code Use

Identify any fixes or improvements you made to your code after reviewing the code provided by the ChatGPT:

1. No changes made.  

### 5.4 Reference

Provide the URL for your discussion in ChatGPT:

https://chat.openai.com/c/d66cd285-259c-439c-bb99-c44b79e0b6c0