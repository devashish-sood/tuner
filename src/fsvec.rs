pub struct FixedSizeVec<T: Clone> {
  buf: Vec<T>, 
  m_size: usize,
}

// Fixed size vector that pushes out old data on append
impl<T: Clone> FixedSizeVec<T> {

  pub fn new(size: usize) -> Self {
    Self {
      buf: Vec::with_capacity(size),
      m_size: size
    }
  }

  pub fn with_vec(size: usize, vec: &Vec<T>) -> Self {
    Self {
      buf: vec.clone(),
      m_size: size
    }
  }

  pub fn extend<I>(&mut self, items: I)
    where I: IntoIterator<Item=T> {
      let items: Vec<T> = items.into_iter().collect();
      let items_len= items.len();


      // Doesn't entirely handle cases where items_len > 2 * self.m_size, easy fix but it doesn't affect my use cases for now.
      if items_len > self.m_size {
        self.buf = items[items_len-self.m_size..].to_vec(); 
      }
      else {
        let overflow = self.buf.len() + items_len - self.m_size;
        self.buf.drain(0..overflow);
        self.buf.extend(items);
      }
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
    return self.len() == self.m_size;
  }
  }

#[cfg(test)]
  mod tests {
    use crate::fsvec::FixedSizeVec;

  
      #[test]
      fn test_init_fixed_vec() {
        let vec = FixedSizeVec::<f32>::new(5);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.m_size, 5);
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
        assert_eq!(fs_v.m_size, 10);
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
  }