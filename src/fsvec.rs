use std::borrow::Borrow;

pub struct FixedSizeVec<T: Clone> {
  buf: Vec<T>, 
  size: usize,
}
// Fixed size vector that pushes out old data on append
#[allow(dead_code)]
impl<T: Clone> FixedSizeVec<T> {

  pub fn new(size: usize) -> Self {
    Self {
      buf: Vec::with_capacity(size),
      size,
    }
  }

  pub fn with_vec(size: usize, vec: &Vec<T>) -> Self {
    assert!(size >= vec.len());
    let mut buf = Vec::with_capacity(size);
    buf.extend(vec.iter().cloned());
    Self {
      buf,
      size: size
    }
  }

  pub fn extend<I>(&mut self, items: I)
    where 
    I: IntoIterator,
    I::Item: Borrow<T>,
    T: ToOwned<Owned = T> 
    {
      let mut iter = items.into_iter();
      let mut count = self.len();

      while !self.full() {
        if let Some(i) = iter.next() {
          self.buf.push(i.borrow().to_owned());
          count += 1;
        }
        else {
          return;
        }
      }

      for i in iter {
        self.buf[count % self.size] = i.borrow().clone();
        count += 1;
      }

      self.buf.rotate_left(count % self.size);
    }

    pub fn as_slice(&self) -> &[T] {
      &self.buf
    }


  pub fn as_mut_slice(&mut self) -> &mut [T] {
      &mut self.buf
    }

  pub fn len(&self) -> usize {
      self.buf.len()
    }

  pub fn full(&self) -> bool {
    return self.len() >= self.size;
  }
  }

#[cfg(test)]
  mod tests {
    use crate::fsvec::FixedSizeVec;

  
      #[test]
      fn test_init_fixed_vec() {
        let vec = FixedSizeVec::<f32>::new(5);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.size, 5);
      }

      #[test]
      fn test_vec_size() {
        let vec = FixedSizeVec::<f32>::new(5);
        assert_eq!(vec.len(), 0);
      }

      #[test]
      fn test_init_with_vec() {
        let v = vec![1, 2, 3, 4, 5];
        let fs_v = FixedSizeVec::with_vec(10, &v);
        assert_eq!(v.len(), fs_v.len());
        assert_eq!(fs_v.size, 10);
      }

      #[test]
      fn test_extension_with_basic_overflow() {
        let v = vec![1, 2, 3, 4, 5];
        let mut fs_v = FixedSizeVec::with_vec(5, &v);
        let v_2 = vec![6, 7, 8];
        fs_v.extend(v_2);
        assert_eq!(fs_v.as_slice(), vec![4, 5, 6, 7, 8]);
      }

      #[test]
      fn test_extension_with_max_overflow() {
        let v = vec![1, 2, 3, 4, 5];
        let mut fs_v = FixedSizeVec::with_vec(5, &v);
        let v_2 = vec![6, 7, 8, 9, 10, 11, 12]; 
        fs_v.extend(v_2);
        assert_eq!(fs_v.as_mut_slice(), vec![8, 9, 10, 11, 12]);
      }

      #[test]
      fn test_extension_with_multiple_overflow() {
        let v = vec![1, 2, 3, 4, 5];
        let mut fs_v = FixedSizeVec::with_vec(5, &v);
        let v_2 = vec![6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]; 
        fs_v.extend(v_2);
        assert_eq!(fs_v.as_mut_slice(), vec![12, 13, 14, 15, 16]);
      }
  }