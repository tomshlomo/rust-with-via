pub struct NonEmptyVec<T> {
    head: T,
    tail: Vec<T>,
}

impl<T> NonEmptyVec<T> {
    pub fn new(item: T) -> Self {
        NonEmptyVec {
            head: item,
            tail: Vec::new(),
        }
    }
    pub fn push(&mut self, item: T) {
        self.tail.push(item);
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        if index == 0 {
            Some(&self.head)
        } else {
            self.tail.get(index - 1)
        }
    }
    pub fn first(&self) -> &T {
        &self.head
    }
    pub fn first_mut(&mut self) -> &mut T {
        &mut self.head
    }
    pub fn last(&self) -> &T {
        self.tail.last().unwrap_or(&self.head)
    }
    pub fn last_mut(&mut self) -> &mut T {
        self.tail.last_mut().unwrap_or(&mut self.head)
    }
    pub fn pop(mut self) -> (T, Vec<T>) {
        match self.tail.pop() {
            Some(item) => (item, self.into()),
            None => (self.head, Vec::new()),
        }
    }
}

impl<T> From<NonEmptyVec<T>> for Vec<T> {
    fn from(x: NonEmptyVec<T>) -> Vec<T> {
        let mut vec = vec![x.head];
        vec.extend(x.tail);
        vec
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let x: NonEmptyVec<i32> = NonEmptyVec::new(5);
        assert_eq!(x.first(), &5);
        assert_eq!(x.last(), &5);
    }

    #[test]
    fn test_push() {
        let mut x: NonEmptyVec<i32> = NonEmptyVec::new(5);
        x.push(10);
        assert_eq!(x.first(), &5);
        assert_eq!(x.last(), &10);
    }

    #[test]
    fn test_get() {
        let mut x: NonEmptyVec<i32> = NonEmptyVec::new(5);
        x.push(10);
        x.push(15);
        assert_eq!(x.get(0), Some(&5));
        assert_eq!(x.get(1), Some(&10));
        assert_eq!(x.get(2), Some(&15));
        assert_eq!(x.get(3), None);
    }

    #[test]
    fn test_first_mut() {
        let mut x: NonEmptyVec<i32> = NonEmptyVec::new(5);
        *x.first_mut() = 10;
        assert_eq!(x.first(), &10);
    }

    #[test]
    fn test_last_mut() {
        let mut x: NonEmptyVec<i32> = NonEmptyVec::new(5);
        x.push(10);
        *x.last_mut() = 15;
        assert_eq!(x.last(), &15);
    }

    #[test]
    fn test_pop() {
        let mut x: NonEmptyVec<i32> = NonEmptyVec::new(5);
        x.push(10);
        x.push(15);
        let (item, y) = x.pop();
        assert_eq!(item, 15);
        assert_eq!(y, vec![5, 10]);
    }

    #[test]
    fn test_pop_single_item() {
        let x: NonEmptyVec<i32> = NonEmptyVec::new(5);
        let (item, y) = x.pop();
        assert_eq!(item, 5);
        assert!(y.is_empty());
    }
}
