---
id: "test-010"
title: "Linked List Library"
state: draft
created_at: "2026-01-15T10:00:00Z"
updated_at: "2026-01-15T10:00:00Z"
---

## Overview

Create a generic linked list library in Rust with both singly and doubly linked list implementations. Provide common operations like insert, delete, search, and iteration. Focus on memory safety and zero-cost abstractions.

## Constraints

- No unsafe code unless absolutely necessary
- Must be no_std compatible

## Implementation Notes

### Crate Configuration
```toml
[package]
name = "linked-list"
version = "0.1.0"
edition = "2021"

[features]
default = ["std"]
std = []

[dependencies]
# no_std compatible - uses alloc crate for Box
```

### Singly Linked List Structure
```rust
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::boxed::Box;
use core::iter::FromIterator;

pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SinglyLinkedList<T> {
    pub const fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }
}
```

### Doubly Linked List (using safe Rc/RefCell)
```rust
use alloc::rc::Rc;
use core::cell::RefCell;

pub struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<DNode<T>>>>,
    tail: Option<Rc<RefCell<DNode<T>>>>,
    len: usize,
}

struct DNode<T> {
    value: T,
    prev: Option<Rc<RefCell<DNode<T>>>>,
    next: Option<Rc<RefCell<DNode<T>>>>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, tail: None, len: 0 }
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(DNode {
            value,
            prev: self.tail.clone(),
            next: None,
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
            }
            None => {
                self.head = Some(new_node.clone());
            }
        }
        self.tail = Some(new_node);
        self.len += 1;
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(DNode {
            value,
            prev: None,
            next: self.head.clone(),
        }));

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
            }
            None => {
                self.tail = Some(new_node.clone());
            }
        }
        self.head = Some(new_node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev = None;
                    self.head = Some(new_head);
                }
                None => {
                    self.tail = None;
                }
            }
            self.len -= 1;
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().value
        })
    }
}
```

### Iterator Implementation
```rust
// Singly linked list iterators
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}

impl<T> SinglyLinkedList<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_deref() }
    }
}

// IntoIterator implementation
impl<T> IntoIterator for SinglyLinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(SinglyLinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}
```

### Drop Implementation (prevent stack overflow)
```rust
impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        // Iterative drop to avoid stack overflow on long lists
        let mut cur = self.head.take();
        while let Some(mut node) = cur {
            cur = node.next.take();
        }
    }
}
```

### Trait Implementations
```rust
impl<T> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for SinglyLinkedList<T> {
    fn clone(&self) -> Self {
        self.iter().cloned().collect()
    }
}

impl<T> FromIterator<T> for SinglyLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for item in iter {
            list.push_front(item);
        }
        // Reverse to maintain order
        list.reverse();
        list
    }
}

impl<T: PartialEq> SinglyLinkedList<T> {
    pub fn contains(&self, value: &T) -> bool {
        self.iter().any(|v| v == value)
    }
}
```

## Review Notes

(none yet)

## Tickets

### Ticket 1: Singly Linked List

**Summary:** Implement singly linked list with basic operations.

**Definition of Done:** Insert, delete, and search operations work correctly.

### Ticket 2: Doubly Linked List

**Summary:** Implement doubly linked list with bidirectional traversal.

**Definition of Done:** Forward and backward iteration works.

### Ticket 3: Iterator Implementation

**Summary:** Implement Iterator trait for both list types.

**Definition of Done:** Lists work with for loops and iterator methods.
