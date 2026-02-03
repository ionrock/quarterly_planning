---
id: "test-010"
title: "Linked List Library"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create a Rust library implementing singly and doubly linked lists with comprehensive functionality. Includes iterators, common operations, and memory-safe implementation.

## Constraints

- Zero unsafe code
- Full test coverage

## Implementation Notes

- Use Rc<RefCell<>> for doubly linked
- Use Box for singly linked
- Implement std traits (Iterator, Debug, Clone)

## Review Notes

(none yet)

## Tickets

### Ticket 1: Singly Linked List

**Summary:** Implement singly linked list with basic operations.

**Definition of Done:** Push, pop, peek, and iteration work correctly.

#### Steps

1. **Create library crate**
   - Run `cargo new linked_list --lib`
   - Verify: `cargo build` succeeds

2. **Define Node struct for singly linked**
   - Create src/singly.rs
   - Define Node<T> with data: T and next: Option<Box<Node<T>>>
   - Verify: struct compiles

3. **Define SinglyLinkedList struct**
   - Define SinglyLinkedList<T> with head: Option<Box<Node<T>>> and len: usize
   - Verify: struct compiles

4. **Implement new() constructor**
   - Return SinglyLinkedList with head: None, len: 0
   - Verify: `SinglyLinkedList::<i32>::new()` works

5. **Implement push_front()**
   - Create new node with data
   - Set new node's next to current head
   - Update head to new node
   - Increment len
   - Verify: push_front adds to front

6. **Implement pop_front()**
   - Return None if empty
   - Take head, update head to head.next
   - Decrement len
   - Return Some(data)
   - Verify: pop_front removes from front

7. **Implement peek_front()**
   - Return Option<&T> referencing head data
   - Verify: peek returns reference without removing

8. **Implement len() and is_empty()**
   - len() returns self.len
   - is_empty() returns self.len == 0
   - Verify: length tracking correct

9. **Implement Iterator trait**
   - Create IntoIter struct wrapping list
   - Implement next() using pop_front()
   - Verify: for loop iteration works

10. **Implement iter() for references**
    - Create Iter struct with current: Option<&Node<T>>
    - Implement next() returning Option<&T>
    - Verify: can iterate without consuming

11. **Implement Debug trait**
    - Format as [a, b, c, ...]
    - Verify: println!("{:?}", list) works

12. **Implement Drop trait**
    - Iteratively drop nodes to avoid stack overflow
    - Verify: large lists deallocate correctly

13. **Write unit tests**
    - Test push/pop sequences
    - Test empty list edge cases
    - Test iteration
    - Verify: `cargo test` passes

### Ticket 2: Doubly Linked List

**Summary:** Implement doubly linked list with bidirectional traversal.

**Definition of Done:** All operations work, no memory leaks.

#### Steps

1. **Define Node struct for doubly linked**
   - Create src/doubly.rs
   - Define Node<T> with data: T, next: Option<Rc<RefCell<Node<T>>>>, prev: Option<Weak<RefCell<Node<T>>>>
   - Verify: struct compiles

2. **Define DoublyLinkedList struct**
   - Define with head: Option<Rc<RefCell<Node<T>>>>, tail: Option<Rc<RefCell<Node<T>>>>, len: usize
   - Verify: struct compiles

3. **Implement new() constructor**
   - Return empty list with head: None, tail: None, len: 0
   - Verify: constructor works

4. **Implement push_front()**
   - Create new node wrapped in Rc<RefCell<>>
   - Link new node to old head
   - Update old head's prev to new node
   - Update head pointer
   - Verify: push_front adds to front

5. **Implement push_back()**
   - Create new node wrapped in Rc<RefCell<>>
   - Link old tail's next to new node
   - Update new node's prev to old tail
   - Update tail pointer
   - Verify: push_back adds to back

6. **Implement pop_front()**
   - Return None if empty
   - Get head node data
   - Update head to head.next
   - Clear new head's prev pointer
   - Verify: pop_front removes from front

7. **Implement pop_back()**
   - Return None if empty
   - Get tail node data
   - Update tail to tail.prev (upgrade Weak)
   - Clear new tail's next pointer
   - Verify: pop_back removes from back

8. **Implement peek_front() and peek_back()**
   - Return references to head/tail data
   - Verify: peek works without mutation

9. **Implement forward iterator**
   - Create Iter struct with current: Option<Rc<RefCell<Node<T>>>>
   - Implement next() traversing forward
   - Verify: forward iteration works

10. **Implement reverse iterator**
    - Create RevIter struct with current: Option<Weak<RefCell<Node<T>>>>
    - Implement next() traversing backward
    - Verify: reverse iteration works

11. **Implement Debug trait**
    - Format as [a <-> b <-> c]
    - Verify: debug output shows structure

12. **Implement Drop trait**
    - Break cycles by clearing next pointers
    - Verify: no memory leaks (use valgrind or miri)

13. **Write comprehensive tests**
    - Test all push/pop combinations
    - Test single element list
    - Test iteration both directions
    - Verify: all tests pass

### Ticket 3: Advanced Operations

**Summary:** Add sorting, searching, and utility methods.

**Definition of Done:** All utility methods implemented and tested.

#### Steps

1. **Implement contains()**
   - Search for value using iteration
   - Return bool
   - Verify: finds existing elements

2. **Implement find()**
   - Search for value, return Option<&T>
   - Verify: returns reference to found element

3. **Implement remove_first()**
   - Remove first occurrence of value
   - Return bool indicating if removed
   - Verify: removes correct element

4. **Implement clear()**
   - Remove all elements
   - Reset head/tail to None
   - Verify: list empty after clear

5. **Implement reverse() for singly linked**
   - Reverse list in place
   - Use three-pointer technique
   - Verify: order reversed correctly

6. **Implement reverse() for doubly linked**
   - Swap next/prev pointers on each node
   - Swap head/tail pointers
   - Verify: order reversed correctly

7. **Implement append()**
   - Append another list to end
   - Take ownership of other list
   - Verify: lists concatenated

8. **Implement split_at()**
   - Split list at index
   - Return new list with second half
   - Verify: split correct at index

9. **Implement Clone trait**
   - Deep clone all nodes
   - Verify: clone is independent copy

10. **Implement From<Vec<T>>**
    - Convert Vec to linked list
    - Verify: conversion works

11. **Implement Into<Vec<T>>**
    - Convert linked list to Vec
    - Verify: conversion works

12. **Implement PartialEq trait**
    - Compare element by element
    - Verify: equality comparison works

13. **Add documentation**
    - Document all public methods
    - Add examples in doc comments
    - Verify: `cargo doc` generates docs

14. **Run clippy and fix warnings**
    - Run `cargo clippy`
    - Fix all warnings
    - Verify: no clippy warnings
