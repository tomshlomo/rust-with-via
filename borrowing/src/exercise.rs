pub struct NonEmptyVec<T> {
    head: T,
    tail: Vec<T>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let x: NonEmptyVec<i32> = NonEmptyVec::new(5);
        todo!("assert that x.first() returns 5, and that x.last() also returns 5.");
    }

    #[test]
    fn test_push() {
        let mut x: NonEmptyVec<i32> = NonEmptyVec::new(5);
        x.push(10);
        todo!("assert that x.first() returns 5, and that x.last() returns 10.");
    }

    #[test]
    fn test_get() {
        let mut x: NonEmptyVec<i32> = NonEmptyVec::new(5);
        x.push(10);
        x.push(15);
        todo!("check the result of `x.get` for indices 0, 1, 2 and 3");
    }

    #[test]
    fn test_first_mut() {
        let mut x: NonEmptyVec<i32> = NonEmptyVec::new(5);
        todo!("use `x.first_mut()` to modify the first item, and then assert that `x.first()` returns the modified value.");
    }

    #[test]
    fn test_last_mut() {
        let mut x: NonEmptyVec<i32> = NonEmptyVec::new(5);
        x.push(10);
        todo!("use `x.last_mut()` to modify the last item, and then assert that `x.last()` returns the modified value.");
    }

    #[test]
    fn test_pop() {
        let mut x: NonEmptyVec<i32> = NonEmptyVec::new(5);
        x.push(10);
        x.push(15);
        todo!("call `x.pop()`, and verify that the popped item is 15, and the remaining values are 5 and 10.");
    }

    #[test]
    fn test_pop_single_item() {
        let x: NonEmptyVec<i32> = NonEmptyVec::new(5);
        todo!("call `x.pop()`, and verify that the popped item is 5, and that there are no remaining values.");
    }
}
