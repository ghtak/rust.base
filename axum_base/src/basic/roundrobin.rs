use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct RoundRobin<T> {
    objects: Vec<T>,
    objects_len: usize,
    index: AtomicUsize,
}

impl<T> RoundRobin<T> {
    pub fn new(objects: Vec<T>) -> Self {
        RoundRobin {
            objects_len: objects.len(),
            objects: objects,
            index: AtomicUsize::new(0),
        }
    }

    pub fn next<'this>(&'this self) -> &'this T {
        if self.objects_len <= 1 {
            return &self.objects[0];
        }
        let mut current = self.index.load(Ordering::Relaxed);
        loop {
            let mut new = current + 1;
            if new >= self.objects_len {
                new = 0;
            }
            match self.index.compare_exchange_weak(
                current,
                new,
                std::sync::atomic::Ordering::Acquire,
                std::sync::atomic::Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(changed) => current = changed,
            }
        }
        return &self.objects[current];
    }
}

#[cfg(test)]
mod tests {
    use super::RoundRobin;

    #[test]
    fn rount_robin_int_vec() {
        let rr = RoundRobin::new(vec![0, 1, 2, 3]);
        for n in 0..12 {
            assert_eq!(n % 4, *rr.next())
        }
    }
}
