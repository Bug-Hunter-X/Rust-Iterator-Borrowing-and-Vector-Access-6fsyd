# Rust Iterator and Vector Access Bug

This repository demonstrates a common error in Rust when working with iterators and vectors.  Attempting to access vector elements by index after using an iterator on the vector can lead to unexpected panics.  The solution shows how to avoid this problem by either consuming the iterator or using different access methods.

## Bug Description

The `bug.rs` file contains code that iterates over a vector using an iterator and then attempts to directly access elements of the vector by index. This results in a runtime panic because the borrow checker cannot guarantee the memory safety of the vector after it's been borrowed by the iterator.

## Solution

The `bugSolution.rs` file provides two solutions:

1. **Consuming the iterator:** The iterator is fully consumed before accessing the vector elements by index, resolving the borrow checker issue.
2. **Using indexing directly:** The code is refactored to directly access elements via indexing to avoid the use of an iterator if iteration is not required.