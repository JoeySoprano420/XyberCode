#[derive(Clone)]
enum RefCounted<T> {
    Value(T),
    RefCount(usize),
}

impl<T> RefCounted<T> {
    fn increment_ref(&mut self) {
        match self {
            RefCounted::RefCount(n) => *n += 1,
            _ => panic!("Cannot increment reference on non-ref count value"),
        }
    }

    fn decrement_ref(&mut self) -> bool {
        match self {
            RefCounted::RefCount(n) => {
                *n -= 1;
                *n == 0
            }
            _ => panic!("Cannot decrement reference on non-ref count value"),
        }
    }
}
