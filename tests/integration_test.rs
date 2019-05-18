#[allow(dead_code)]
mod test_util {
    use heap_in_rust::Heap;

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

    pub fn assert_non_empty_heap(heap: &Heap, top_element: i32) {
        assert!(!heap.is_empty());

        let peek_result = heap.peek();

        assert_value_result(peek_result, top_element);
    }

    pub fn assert_empty_heap(heap: &Heap) {
        assert!(heap.is_empty());

        assert_peek_on_empty_heap(&heap);
    }
}

#[cfg(test)]
mod min_heap_tests {
    use super::test_util;
    use heap_in_rust::{Heap, HeapProperty};

    #[test]
    fn add_six_elements_as_random() {
        let mut heap = Heap::new_as(6, HeapProperty::Min);

        let result = heap.add(5);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 5);

        let result = heap.add(3);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 3);

        let result = heap.add(9);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 3);

        let result = heap.add(8);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 3);

        let result = heap.add(1);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 1);

        let result = heap.add(6);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 1);

        let result = heap.add(0);

        assert!(result.is_err());
        assert_eq!("Full heap.".to_string(), result.unwrap_err());
        test_util::assert_non_empty_heap(&heap, 1);
    }

    #[test]
    fn add_six_elements_as_sequential() {
        let mut heap = Heap::new_as(6, HeapProperty::Min);

        let result = heap.add(1);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 1);

        let result = heap.add(2);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 1);

        let result = heap.add(3);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 1);

        let result = heap.add(4);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 1);

        let result = heap.add(5);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 1);

        let result = heap.add(6);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 1);

        let result = heap.add(0);

        assert!(result.is_err());
        assert_eq!("Full heap.".to_string(), result.unwrap_err());
        test_util::assert_non_empty_heap(&heap, 1);
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
        test_util::assert_non_empty_heap(&heap, 1);

        let result = heap.pop();

        test_util::assert_value_result(result, 1);
        test_util::assert_non_empty_heap(&heap, 3);

        let result = heap.pop();

        test_util::assert_value_result(result, 3);
        test_util::assert_non_empty_heap(&heap, 5);

        let result = heap.pop();

        test_util::assert_value_result(result, 5);
        test_util::assert_non_empty_heap(&heap, 6);

        let result = heap.pop();

        test_util::assert_value_result(result, 6);
        test_util::assert_non_empty_heap(&heap, 8);

        let result = heap.pop();

        test_util::assert_value_result(result, 8);
        test_util::assert_non_empty_heap(&heap, 9);

        let result = heap.pop();

        test_util::assert_value_result(result, 9);
        test_util::assert_empty_heap(&heap);

        let result = heap.pop();

        assert!(result.is_err());
        assert_eq!("Empty heap.".to_string(), result.unwrap_err());
        test_util::assert_empty_heap(&heap);
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
        test_util::assert_non_empty_heap(&heap, 1);

        let result = heap.pop();

        test_util::assert_value_result(result, 1);
        test_util::assert_non_empty_heap(&heap, 2);

        let result = heap.pop();

        test_util::assert_value_result(result, 2);
        test_util::assert_non_empty_heap(&heap, 3);

        let result = heap.pop();

        test_util::assert_value_result(result, 3);
        test_util::assert_non_empty_heap(&heap, 4);

        let result = heap.pop();

        test_util::assert_value_result(result, 4);
        test_util::assert_non_empty_heap(&heap, 5);

        let result = heap.pop();

        test_util::assert_value_result(result, 5);
        test_util::assert_non_empty_heap(&heap, 6);

        let result = heap.pop();

        test_util::assert_value_result(result, 6);
        test_util::assert_empty_heap(&heap);

        let result = heap.pop();

        assert!(result.is_err());
        assert_eq!("Empty heap.".to_string(), result.unwrap_err());
        test_util::assert_empty_heap(&heap);
    }
}

#[cfg(test)]
mod max_heap_tests {
    use super::test_util;
    use heap_in_rust::{Heap, HeapProperty};

    #[test]
    fn add_six_elements_as_random() {
        let mut heap = Heap::new_as(6, HeapProperty::Max);

        let result = heap.add(5);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 5);

        let result = heap.add(3);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 5);

        let result = heap.add(9);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 9);

        let result = heap.add(8);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 9);

        let result = heap.add(1);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 9);

        let result = heap.add(6);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 9);

        let result = heap.add(0);

        assert!(result.is_err());
        assert_eq!("Full heap.".to_string(), result.unwrap_err());
        test_util::assert_non_empty_heap(&heap, 9);
    }

    #[test]
    fn add_six_elements_as_sequential() {
        let mut heap = Heap::new_as(6, HeapProperty::Max);

        let result = heap.add(1);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 1);

        let result = heap.add(2);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 2);

        let result = heap.add(3);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 3);

        let result = heap.add(4);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 4);

        let result = heap.add(5);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 5);

        let result = heap.add(6);

        test_util::assert_ok_result(result);
        test_util::assert_non_empty_heap(&heap, 6);

        let result = heap.add(0);

        assert!(result.is_err());
        assert_eq!("Full heap.".to_string(), result.unwrap_err());
        test_util::assert_non_empty_heap(&heap, 6);
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
        test_util::assert_non_empty_heap(&heap, 9);

        let result = heap.pop();

        test_util::assert_value_result(result, 9);
        test_util::assert_non_empty_heap(&heap, 8);

        let result = heap.pop();

        test_util::assert_value_result(result, 8);
        test_util::assert_non_empty_heap(&heap, 6);

        let result = heap.pop();

        test_util::assert_value_result(result, 6);
        test_util::assert_non_empty_heap(&heap, 5);

        let result = heap.pop();

        test_util::assert_value_result(result, 5);
        test_util::assert_non_empty_heap(&heap, 3);

        let result = heap.pop();

        test_util::assert_value_result(result, 3);
        test_util::assert_non_empty_heap(&heap, 1);

        let result = heap.pop();

        test_util::assert_value_result(result, 1);
        test_util::assert_empty_heap(&heap);

        let result = heap.pop();

        assert!(result.is_err());
        assert_eq!("Empty heap.".to_string(), result.unwrap_err());
        test_util::assert_empty_heap(&heap);
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
        test_util::assert_non_empty_heap(&heap, 6);

        let result = heap.pop();

        test_util::assert_value_result(result, 6);
        test_util::assert_non_empty_heap(&heap, 5);

        let result = heap.pop();

        test_util::assert_value_result(result, 5);
        test_util::assert_non_empty_heap(&heap, 4);

        let result = heap.pop();

        test_util::assert_value_result(result, 4);
        test_util::assert_non_empty_heap(&heap, 3);

        let result = heap.pop();

        test_util::assert_value_result(result, 3);
        test_util::assert_non_empty_heap(&heap, 2);

        let result = heap.pop();

        test_util::assert_value_result(result, 2);
        test_util::assert_non_empty_heap(&heap, 1);

        let result = heap.pop();

        test_util::assert_value_result(result, 1);
        test_util::assert_empty_heap(&heap);

        let result = heap.pop();

        assert!(result.is_err());
        assert_eq!("Empty heap.".to_string(), result.unwrap_err());
        test_util::assert_empty_heap(&heap);
    }
}
