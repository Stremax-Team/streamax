use crate::core::{Result, Error, Serialize};

/// Map implementation for key-value storage
pub struct Map<K, V> {
    prefix: Vec<u8>,
    _phantom: std::marker::PhantomData<(K, V)>,
}

impl<K: Serialize, V: Serialize> Map<K, V> {
    pub fn new(namespace: &[u8]) -> Self {
        Map {
            prefix: namespace.to_vec(),
            _phantom: std::marker::PhantomData,
        }
    }
    
    pub fn get(&self, key: &K) -> Result<Option<V>> {
        let key_bytes = key.serialize();
        let storage_key = [&self.prefix[..], &key_bytes[..]].concat();
        
        match storage::get(&storage_key) {
            Some(data) => {
                V::deserialize(&data)
                    .map(Some)
                    .map_err(|_| Error::SerializationError)
            }
            None => Ok(None)
        }
    }
    
    pub fn set(&mut self, key: &K, value: &V) -> Result<()> {
        let key_bytes = key.serialize();
        let value_bytes = value.serialize();
        let storage_key = [&self.prefix[..], &key_bytes[..]].concat();
        
        storage::set(&storage_key, &value_bytes);
        Ok(())
    }
    
    pub fn remove(&mut self, key: &K) -> Result<()> {
        let key_bytes = key.serialize();
        let storage_key = [&self.prefix[..], &key_bytes[..]].concat();
        
        storage::remove(&storage_key);
        Ok(())
    }
    
    pub fn contains(&self, key: &K) -> Result<bool> {
        let key_bytes = key.serialize();
        let storage_key = [&self.prefix[..], &key_bytes[..]].concat();
        
        Ok(storage::contains(&storage_key))
    }
}

/// Set implementation for unique value storage
pub struct Set<T> {
    inner: Map<T, bool>,
}

impl<T: Serialize> Set<T> {
    pub fn new(namespace: &[u8]) -> Self {
        Set {
            inner: Map::new(namespace),
        }
    }
    
    pub fn insert(&mut self, value: &T) -> Result<bool> {
        if self.contains(value)? {
            return Ok(false);
        }
        self.inner.set(value, &true)?;
        Ok(true)
    }
    
    pub fn remove(&mut self, value: &T) -> Result<bool> {
        if !self.contains(value)? {
            return Ok(false);
        }
        self.inner.remove(value)?;
        Ok(true)
    }
    
    pub fn contains(&self, value: &T) -> Result<bool> {
        self.inner.contains(value)
    }
}

/// Vector implementation for sequential storage
pub struct Vec<T> {
    prefix: Vec<u8>,
    length: u64,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Serialize> Vec<T> {
    pub fn new(namespace: &[u8]) -> Self {
        let mut vec = Vec {
            prefix: namespace.to_vec(),
            length: 0,
            _phantom: std::marker::PhantomData,
        };
        
        // Load length from storage
        let length_key = [&namespace[..], b"_length"].concat();
        if let Some(len_bytes) = storage::get(&length_key) {
            vec.length = u64::from_be_bytes(len_bytes.try_into().unwrap_or([0; 8]));
        }
        
        vec
    }
    
    pub fn push(&mut self, value: &T) -> Result<()> {
        let value_bytes = value.serialize();
        let index_key = [&self.prefix[..], &self.length.to_be_bytes()].concat();
        
        storage::set(&index_key, &value_bytes);
        self.length += 1;
        
        // Update length in storage
        let length_key = [&self.prefix[..], b"_length"].concat();
        storage::set(&length_key, &self.length.to_be_bytes());
        
        Ok(())
    }
    
    pub fn pop(&mut self) -> Result<Option<T>> {
        if self.length == 0 {
            return Ok(None);
        }
        
        self.length -= 1;
        let index_key = [&self.prefix[..], &self.length.to_be_bytes()].concat();
        
        let value = match storage::get(&index_key) {
            Some(data) => {
                T::deserialize(&data)
                    .map_err(|_| Error::SerializationError)?
            }
            None => return Ok(None)
        };
        
        storage::remove(&index_key);
        
        // Update length in storage
        let length_key = [&self.prefix[..], b"_length"].concat();
        storage::set(&length_key, &self.length.to_be_bytes());
        
        Ok(Some(value))
    }
    
    pub fn get(&self, index: u64) -> Result<Option<T>> {
        if index >= self.length {
            return Ok(None);
        }
        
        let index_key = [&self.prefix[..], &index.to_be_bytes()].concat();
        match storage::get(&index_key) {
            Some(data) => {
                T::deserialize(&data)
                    .map(Some)
                    .map_err(|_| Error::SerializationError)
            }
            None => Ok(None)
        }
    }
    
    pub fn len(&self) -> u64 {
        self.length
    }
    
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

/// Queue implementation for FIFO operations
pub struct Queue<T> {
    inner: Vec<T>,
}

impl<T: Serialize> Queue<T> {
    pub fn new(namespace: &[u8]) -> Self {
        Queue {
            inner: Vec::new(namespace),
        }
    }
    
    pub fn enqueue(&mut self, value: &T) -> Result<()> {
        self.inner.push(value)
    }
    
    pub fn dequeue(&mut self) -> Result<Option<T>> {
        if self.inner.is_empty() {
            return Ok(None);
        }
        
        // Get first element
        let value = self.inner.get(0)?.unwrap();
        
        // Shift all elements left
        for i in 1..self.inner.len() {
            if let Some(next) = self.inner.get(i)? {
                let prev_key = [&self.inner.prefix[..], &(i-1).to_be_bytes()].concat();
                let next_bytes = next.serialize();
                storage::set(&prev_key, &next_bytes);
            }
        }
        
        // Update length
        self.inner.length -= 1;
        let length_key = [&self.inner.prefix[..], b"_length"].concat();
        storage::set(&length_key, &self.inner.length.to_be_bytes());
        
        Ok(Some(value))
    }
    
    pub fn peek(&self) -> Result<Option<T>> {
        self.inner.get(0)
    }
    
    pub fn len(&self) -> u64 {
        self.inner.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_map() {
        let mut map: Map<String, u64> = Map::new(b"test_map");
        
        // Test set and get
        map.set(&"key1".to_string(), &42).unwrap();
        assert_eq!(map.get(&"key1".to_string()).unwrap(), Some(42));
        
        // Test remove
        map.remove(&"key1".to_string()).unwrap();
        assert_eq!(map.get(&"key1".to_string()).unwrap(), None);
        
        // Test contains
        map.set(&"key2".to_string(), &100).unwrap();
        assert!(map.contains(&"key2".to_string()).unwrap());
        assert!(!map.contains(&"key1".to_string()).unwrap());
    }
    
    #[test]
    fn test_set() {
        let mut set: Set<String> = Set::new(b"test_set");
        
        // Test insert
        assert!(set.insert(&"value1".to_string()).unwrap());
        assert!(!set.insert(&"value1".to_string()).unwrap());
        
        // Test contains
        assert!(set.contains(&"value1".to_string()).unwrap());
        assert!(!set.contains(&"value2".to_string()).unwrap());
        
        // Test remove
        assert!(set.remove(&"value1".to_string()).unwrap());
        assert!(!set.remove(&"value1".to_string()).unwrap());
    }
    
    #[test]
    fn test_vec() {
        let mut vec: Vec<u64> = Vec::new(b"test_vec");
        
        // Test push and get
        vec.push(&1).unwrap();
        vec.push(&2).unwrap();
        vec.push(&3).unwrap();
        
        assert_eq!(vec.get(0).unwrap(), Some(1));
        assert_eq!(vec.get(1).unwrap(), Some(2));
        assert_eq!(vec.get(2).unwrap(), Some(3));
        
        // Test pop
        assert_eq!(vec.pop().unwrap(), Some(3));
        assert_eq!(vec.len(), 2);
        
        // Test length
        assert!(!vec.is_empty());
        assert_eq!(vec.len(), 2);
    }
    
    #[test]
    fn test_queue() {
        let mut queue: Queue<String> = Queue::new(b"test_queue");
        
        // Test enqueue
        queue.enqueue(&"first".to_string()).unwrap();
        queue.enqueue(&"second".to_string()).unwrap();
        queue.enqueue(&"third".to_string()).unwrap();
        
        // Test peek
        assert_eq!(queue.peek().unwrap().unwrap(), "first");
        
        // Test dequeue
        assert_eq!(queue.dequeue().unwrap().unwrap(), "first");
        assert_eq!(queue.dequeue().unwrap().unwrap(), "second");
        assert_eq!(queue.len(), 1);
        
        // Test empty
        assert!(!queue.is_empty());
        queue.dequeue().unwrap();
        assert!(queue.is_empty());
    }
} 