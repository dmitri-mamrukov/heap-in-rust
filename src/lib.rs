#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HeapProperty {
    Min,
    Max,
}

pub struct Heap {
    elements: Vec<i32>,
    size: usize,
    property: HeapProperty,
}

impl Heap {
    pub fn new(size: usize) -> Self {
        Heap {
            elements: vec![0; size],
            size: 0,
            property: HeapProperty::Min,
        }
    }

    pub fn new_as(size: usize, property: HeapProperty) -> Self {
        Heap {
            elements: vec![0; size],
            size: 0,
            property,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn peek(&self) -> Result<i32, String> {
        self.validate_non_empty_heap()?;

        Ok(self.elements[0])
    }

    pub fn pop(&mut self) -> Result<i32, String> {
        self.validate_non_empty_heap()?;

        let result = self.elements[0];
        self.elements[0] = self.elements[(self.size - 1)];
        self.size -= 1;

        self.recalculate_down();

        Ok(result)
    }

    pub fn add(&mut self, element: i32) -> Result<(), String> {
        if self.size == self.elements.len() {
            return Err("Full heap.".to_string());
        }

        self.elements[self.size] = element;
        self.size += 1;

        self.recalculate_up();

        Ok(())
    }

    fn validate_non_empty_heap(&self) -> Result<(), String> {
        if self.size == 0 {
            return Err("Empty heap.".to_string());
        }

        Ok(())
    }

    fn get_parent_index(&self, index: usize) -> usize {
        (index - 1) / 2
    }

    fn get_left_child_index(&self, index: usize) -> usize {
        2 * index + 1
    }

    fn get_right_child_index(&self, index: usize) -> usize {
        2 * index + 2
    }

    fn is_root(&self, index: usize) -> bool {
        index == 0
    }

    fn has_left_child(&self, index: usize) -> bool {
        self.get_left_child_index(index) < self.size
    }

    fn has_right_child(&self, index: usize) -> bool {
        self.get_right_child_index(index) < self.size
    }

    fn get_parent(&self, index: usize) -> i32 {
        self.elements[self.get_parent_index(index)]
    }

    fn get_left_child(&self, index: usize) -> i32 {
        self.elements[self.get_left_child_index(index)]
    }

    fn get_right_child(&self, index: usize) -> i32 {
        self.elements[self.get_right_child_index(index)]
    }

    fn swap(&mut self, index1: usize, index2: usize) {
        self.elements.swap(index1, index2);
    }

    fn recalculate_down(&mut self) {
        let mut index = 0_usize;
        while self.has_left_child(index) {
            let candidate_index = if self.has_right_child(index)
                && match self.property {
                    HeapProperty::Min => self.get_right_child(index) < self.get_left_child(index),
                    HeapProperty::Max => self.get_right_child(index) > self.get_left_child(index),
                } {
                self.get_right_child_index(index)
            } else {
                self.get_left_child_index(index)
            };

            if match self.property {
                HeapProperty::Min => self.elements[candidate_index] >= self.elements[index],
                HeapProperty::Max => self.elements[candidate_index] < self.elements[index],
            } {
                break;
            }

            self.swap(candidate_index, index);
            index = candidate_index;
        }
    }

    fn recalculate_up(&mut self) {
        let mut index = self.size - 1;
        while !self.is_root(index)
            && match self.property {
                HeapProperty::Min => self.elements[index] < self.get_parent(index),
                HeapProperty::Max => self.elements[index] > self.get_parent(index),
            }
        {
            let parent_index = self.get_parent_index(index);
            self.swap(parent_index, index);
            index = parent_index;
        }
    }
}

#[cfg(test)]
mod heap_constructor_tests {
    use super::{Heap, HeapProperty};

    fn create_heap(size: usize) -> Heap {
        Heap::new(size)
    }

    fn create_heap_as(size: usize, property: HeapProperty) -> Heap {
        Heap::new_as(size, property)
    }

    fn assert_constructor(heap: &Heap, size: usize, expected_property: HeapProperty) {
        assert_eq!(expected_property, heap.property);
        assert_eq!(size, heap.elements.len());
        assert_eq!(0, heap.size);
        assert!(heap.is_empty());
        for element in heap.elements.iter() {
            assert_eq!(0, *element);
        }
    }

    #[test]
    fn default_constructor_with_zero() {
        let size = 0;
        let heap = create_heap(size);
        assert_constructor(&heap, size, HeapProperty::Min);
    }

    #[test]
    fn default_constructor_with_one() {
        let size = 1;
        let heap = create_heap(size);
        assert_constructor(&heap, size, HeapProperty::Min);
    }

    #[test]
    fn default_constructor_with_five() {
        let size = 5;
        let heap = create_heap(size);
        assert_constructor(&heap, size, HeapProperty::Min);
    }

    #[test]
    fn constructor_with_zero_and_min() {
        let size = 0;
        let property = HeapProperty::Min;
        let heap = create_heap_as(size, property);
        assert_constructor(&heap, size, property);
    }

    #[test]
    fn default_constructor_with_one_and_min() {
        let size = 1;
        let property = HeapProperty::Min;
        let heap = create_heap_as(size, property);
        assert_constructor(&heap, size, property);
    }

    #[test]
    fn default_constructor_with_five_and_min() {
        let size = 5;
        let property = HeapProperty::Min;
        let heap = create_heap_as(size, property);
        assert_constructor(&heap, size, property);
    }
}

#[allow(dead_code)]
mod test_util {
    use super::Heap;

    pub fn assert_peek_on_empty_heap(heap: &Heap) {
        let result = heap.peek();

        assert!(result.is_err());
        assert_eq!("Empty heap.".to_string(), result.unwrap_err());
    }

    pub fn assert_ok_result(result: Result<(), String>) {
        assert!(result.is_ok());
        assert_eq!((), result.unwrap());
    }

    pub fn assert_value_result(result: Result<i32, String>, value: i32) {
        assert!(result.is_ok());
        assert_eq!(value, result.unwrap());
    }

    pub fn assert_non_empty_heap(heap: &Heap, elements: Vec<i32>, size: usize) {
        assert_eq!(elements.capacity(), heap.elements.len());
        assert_eq!(elements, heap.elements);
        assert_eq!(size, heap.size);
        assert!(!heap.is_empty());

        let peek_result = heap.peek();

        assert_value_result(peek_result, elements[0]);
    }

    pub fn assert_empty_heap(heap: &Heap, elements: Vec<i32>) {
        assert_eq!(elements.capacity(), heap.elements.len());
        assert_eq!(elements, heap.elements);
        assert_eq!(0, heap.size);
        assert!(heap.is_empty());

        assert_peek_on_empty_heap(&heap);
    }
}

#[cfg(test)]
mod min_heap_tests {
    use super::test_util;
    use super::{Heap, HeapProperty};

    #[test]
    fn peek_on_heap_with_size_as_zero() {
        let heap = Heap::new_as(0, HeapProperty::Min);
        test_util::assert_peek_on_empty_heap(&heap);
    }

    #[test]
    fn peek_on_heap_with_size_as_five() {
        let heap = Heap::new_as(5, HeapProperty::Min);
        test_util::assert_peek_on_empty_heap(&heap);
    }

    #[test]
    fn add_six_elements_as_random() {
        let mut heap = Heap::new_as(6, HeapProperty::Min);

        let result = heap.add(5);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![5, 0, 0, 0, 0, 0], 1);

        let result = heap.add(3);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![3, 5, 0, 0, 0, 0], 2);

        let result = heap.add(9);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![3, 5, 9, 0, 0, 0], 3);

        let result = heap.add(8);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![3, 5, 9, 8, 0, 0], 4);

        let result = heap.add(1);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![1, 3, 9, 8, 5, 0], 5);

        let result = heap.add(6);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![1, 3, 6, 8, 5, 9], 6);

        let result = heap.add(0);

        assert!(result.is_err());
        assert_eq!("Full heap.".to_string(), result.unwrap_err());
        test_util::assert_non_empty_heap(&heap, vec![1, 3, 6, 8, 5, 9], 6);
    }

    #[test]
    fn add_six_elements_as_sequential() {
        let mut heap = Heap::new_as(6, HeapProperty::Min);

        let result = heap.add(1);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![1, 0, 0, 0, 0, 0], 1);

        let result = heap.add(2);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![1, 2, 0, 0, 0, 0], 2);

        let result = heap.add(3);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![1, 2, 3, 0, 0, 0], 3);

        let result = heap.add(4);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![1, 2, 3, 4, 0, 0], 4);

        let result = heap.add(5);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![1, 2, 3, 4, 5, 0], 5);

        let result = heap.add(6);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![1, 2, 3, 4, 5, 6], 6);

        let result = heap.add(0);

        assert!(result.is_err());
        assert_eq!("Full heap.".to_string(), result.unwrap_err());
        test_util::assert_non_empty_heap(&heap, vec![1, 2, 3, 4, 5, 6], 6);
    }

    #[test]
    fn pop_from_six_elements_as_random() {
        let mut heap = Heap::new_as(6, HeapProperty::Min);
        heap.add(5).unwrap();
        heap.add(3).unwrap();
        heap.add(9).unwrap();
        heap.add(8).unwrap();
        heap.add(1).unwrap();
        heap.add(6).unwrap();
        test_util::assert_non_empty_heap(&heap, vec![1, 3, 6, 8, 5, 9], 6);

        let result = heap.pop();

        test_util::assert_value_result(result, 1);
        test_util::assert_non_empty_heap(&heap, vec![3, 5, 6, 8, 9, 9], 5);

        let result = heap.pop();

        test_util::assert_value_result(result, 3);
        test_util::assert_non_empty_heap(&heap, vec![5, 8, 6, 9, 9, 9], 4);

        let result = heap.pop();

        test_util::assert_value_result(result, 5);
        test_util::assert_non_empty_heap(&heap, vec![6, 8, 9, 9, 9, 9], 3);

        let result = heap.pop();

        test_util::assert_value_result(result, 6);
        test_util::assert_non_empty_heap(&heap, vec![8, 9, 9, 9, 9, 9], 2);

        let result = heap.pop();

        test_util::assert_value_result(result, 8);
        test_util::assert_non_empty_heap(&heap, vec![9, 9, 9, 9, 9, 9], 1);

        let result = heap.pop();

        test_util::assert_value_result(result, 9);
        test_util::assert_empty_heap(&heap, vec![9, 9, 9, 9, 9, 9]);

        let result = heap.pop();

        assert!(result.is_err());
        assert_eq!("Empty heap.".to_string(), result.unwrap_err());
        test_util::assert_empty_heap(&heap, vec![9, 9, 9, 9, 9, 9]);
    }

    #[test]
    fn pop_from_six_elements_as_sequential() {
        let mut heap = Heap::new_as(6, HeapProperty::Min);
        heap.add(1).unwrap();
        heap.add(2).unwrap();
        heap.add(3).unwrap();
        heap.add(4).unwrap();
        heap.add(5).unwrap();
        heap.add(6).unwrap();
        test_util::assert_non_empty_heap(&heap, vec![1, 2, 3, 4, 5, 6], 6);

        let result = heap.pop();

        test_util::assert_value_result(result, 1);
        test_util::assert_non_empty_heap(&heap, vec![2, 4, 3, 6, 5, 6], 5);

        let result = heap.pop();

        test_util::assert_value_result(result, 2);
        test_util::assert_non_empty_heap(&heap, vec![3, 4, 5, 6, 5, 6], 4);

        let result = heap.pop();

        test_util::assert_value_result(result, 3);
        test_util::assert_non_empty_heap(&heap, vec![4, 6, 5, 6, 5, 6], 3);

        let result = heap.pop();

        test_util::assert_value_result(result, 4);
        test_util::assert_non_empty_heap(&heap, vec![5, 6, 5, 6, 5, 6], 2);

        let result = heap.pop();

        test_util::assert_value_result(result, 5);
        test_util::assert_non_empty_heap(&heap, vec![6, 6, 5, 6, 5, 6], 1);

        let result = heap.pop();

        test_util::assert_value_result(result, 6);
        test_util::assert_empty_heap(&heap, vec![6, 6, 5, 6, 5, 6]);

        let result = heap.pop();

        assert!(result.is_err());
        assert_eq!("Empty heap.".to_string(), result.unwrap_err());
        test_util::assert_empty_heap(&heap, vec![6, 6, 5, 6, 5, 6]);
    }
}

#[cfg(test)]
mod max_heap_tests {
    use super::test_util;
    use super::{Heap, HeapProperty};

    #[test]
    fn peek_on_heap_with_size_as_zero() {
        let heap = Heap::new_as(0, HeapProperty::Max);
        test_util::assert_peek_on_empty_heap(&heap);
    }

    #[test]
    fn peek_on_heap_with_size_as_five() {
        let heap = Heap::new_as(5, HeapProperty::Max);
        test_util::assert_peek_on_empty_heap(&heap);
    }

    #[test]
    fn add_six_elements_as_random() {
        let mut heap = Heap::new_as(6, HeapProperty::Max);

        let result = heap.add(5);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![5, 0, 0, 0, 0, 0], 1);

        let result = heap.add(3);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![5, 3, 0, 0, 0, 0], 2);

        let result = heap.add(9);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![9, 3, 5, 0, 0, 0], 3);

        let result = heap.add(8);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![9, 8, 5, 3, 0, 0], 4);

        let result = heap.add(1);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![9, 8, 5, 3, 1, 0], 5);

        let result = heap.add(6);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![9, 8, 6, 3, 1, 5], 6);

        let result = heap.add(0);

        assert!(result.is_err());
        assert_eq!("Full heap.".to_string(), result.unwrap_err());
        test_util::assert_non_empty_heap(&heap, vec![9, 8, 6, 3, 1, 5], 6);
    }

    #[test]
    fn add_six_elements_as_sequential() {
        let mut heap = Heap::new_as(6, HeapProperty::Max);

        let result = heap.add(1);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![1, 0, 0, 0, 0, 0], 1);

        let result = heap.add(2);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![2, 1, 0, 0, 0, 0], 2);

        let result = heap.add(3);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![3, 1, 2, 0, 0, 0], 3);

        let result = heap.add(4);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![4, 3, 2, 1, 0, 0], 4);

        let result = heap.add(5);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![5, 4, 2, 1, 3, 0], 5);

        let result = heap.add(6);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, vec![6, 4, 5, 1, 3, 2], 6);

        let result = heap.add(0);

        assert!(result.is_err());
        assert_eq!("Full heap.".to_string(), result.unwrap_err());
        test_util::assert_non_empty_heap(&heap, vec![6, 4, 5, 1, 3, 2], 6);
    }

    #[test]
    fn pop_from_six_elements_as_random() {
        let mut heap = Heap::new_as(6, HeapProperty::Max);
        heap.add(5).unwrap();
        heap.add(3).unwrap();
        heap.add(9).unwrap();
        heap.add(8).unwrap();
        heap.add(1).unwrap();
        heap.add(6).unwrap();
        test_util::assert_non_empty_heap(&heap, vec![9, 8, 6, 3, 1, 5], 6);

        let result = heap.pop();

        test_util::assert_value_result(result, 9);
        test_util::assert_non_empty_heap(&heap, vec![8, 5, 6, 3, 1, 5], 5);

        let result = heap.pop();

        test_util::assert_value_result(result, 8);
        test_util::assert_non_empty_heap(&heap, vec![6, 5, 1, 3, 1, 5], 4);

        let result = heap.pop();

        test_util::assert_value_result(result, 6);
        test_util::assert_non_empty_heap(&heap, vec![5, 3, 1, 3, 1, 5], 3);

        let result = heap.pop();

        test_util::assert_value_result(result, 5);
        test_util::assert_non_empty_heap(&heap, vec![3, 1, 1, 3, 1, 5], 2);

        let result = heap.pop();

        test_util::assert_value_result(result, 3);
        test_util::assert_non_empty_heap(&heap, vec![1, 1, 1, 3, 1, 5], 1);

        let result = heap.pop();

        test_util::assert_value_result(result, 1);
        test_util::assert_empty_heap(&heap, vec![1, 1, 1, 3, 1, 5]);

        let result = heap.pop();

        assert!(result.is_err());
        assert_eq!("Empty heap.".to_string(), result.unwrap_err());
        test_util::assert_empty_heap(&heap, vec![1, 1, 1, 3, 1, 5]);
    }

    #[test]
    fn pop_from_six_elements_as_sequential() {
        let mut heap = Heap::new_as(6, HeapProperty::Max);
        heap.add(1).unwrap();
        heap.add(2).unwrap();
        heap.add(3).unwrap();
        heap.add(4).unwrap();
        heap.add(5).unwrap();
        heap.add(6).unwrap();
        test_util::assert_non_empty_heap(&heap, vec![6, 4, 5, 1, 3, 2], 6);

        let result = heap.pop();

        test_util::assert_value_result(result, 6);
        test_util::assert_non_empty_heap(&heap, vec![5, 4, 2, 1, 3, 2], 5);

        let result = heap.pop();

        test_util::assert_value_result(result, 5);
        test_util::assert_non_empty_heap(&heap, vec![4, 3, 2, 1, 3, 2], 4);

        let result = heap.pop();

        test_util::assert_value_result(result, 4);
        test_util::assert_non_empty_heap(&heap, vec![3, 1, 2, 1, 3, 2], 3);

        let result = heap.pop();

        test_util::assert_value_result(result, 3);
        test_util::assert_non_empty_heap(&heap, vec![2, 1, 2, 1, 3, 2], 2);

        let result = heap.pop();

        test_util::assert_value_result(result, 2);
        test_util::assert_non_empty_heap(&heap, vec![1, 1, 2, 1, 3, 2], 1);

        let result = heap.pop();

        test_util::assert_value_result(result, 1);
        test_util::assert_empty_heap(&heap, vec![1, 1, 2, 1, 3, 2]);

        let result = heap.pop();

        assert!(result.is_err());
        assert_eq!("Empty heap.".to_string(), result.unwrap_err());
        test_util::assert_empty_heap(&heap, vec![1, 1, 2, 1, 3, 2]);
    }
}
