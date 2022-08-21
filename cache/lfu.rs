use std::collections::{HashMap, HashSet, LinkedList};

#[derive(Debug)]
struct Value {
    value: i32,
    use_counter: i32
}

impl Value {
    fn update_counter(&mut self) -> i32 {
        self.use_counter += 1;
        self.use_counter
    }
    
    fn update(&mut self, value: i32) -> i32 {
        self.value = value;
        self.use_counter += 1;
        self.use_counter
    }
}

#[derive(Debug)]
struct LFUCache {
    data: HashMap<i32, Value>,
    capacity: i32,
    current_capacity: i32,
    usage: HashMap<i32, LinkedList<i32>>,
    least_count: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LFUCache {

    fn new(capacity: i32) -> Self {
        LFUCache {
            data: HashMap::new(),
            capacity: capacity,
            current_capacity: 0 as i32,
            usage: HashMap::new(),
            least_count: 999,
        }
        
    }
    
    fn update_usage_counter(&mut self, use_counter: i32, key: i32) {
        // push use_counter in usage
        let mut counter_arr = self.usage.entry(use_counter).or_insert(LinkedList::new());
        (*counter_arr).push_back(key);
        
        // remove key from previous use_counter
        let prev_use_counter = use_counter - 1;
        let mut counters_is_empty: bool = false;
        // this is O(n) solution
        match self.usage.get_mut(&prev_use_counter) {
            Some(counters) => {
                (*counters).retain(|&x| x != key);
                counters_is_empty = counters.is_empty();
                if counters_is_empty && self.least_count + 1 == use_counter {
                    self.least_count = use_counter;
                }
            },
            None => {},
        };
        
        if counters_is_empty {
            self.usage.remove(&prev_use_counter);
        }
        
        if self.least_count > use_counter {
            self.least_count = use_counter;
        }
    }
    
    fn get(&mut self, key: i32) -> i32 {
        if self.capacity < 1 {
            return -1 as i32;
        }
        let mut use_counter: i32 = 0;
        let val = match self.data.get_mut(&key) {
            Some(x) => {
                use_counter = x.update_counter();
                x.value
            },
            None => -1
        };
        if use_counter > 0 {
            self.update_usage_counter(use_counter, key);
        }
        // println!("GET {} {:?} {:?}", self.least_count, self.usage, self.data);
        val
    }
    
    fn pop_entry(&mut self) {
        let mut counters_is_empty: bool = false;
        let existing_least_count = self.least_count;
        match self.usage.get_mut(&self.least_count) {
            Some(counters) => {
                let key = (*counters).remove(0 as usize);
                self.data.remove(&key);
                self.current_capacity -= 1;
                counters_is_empty = counters.is_empty();
                if counters_is_empty {
                    loop {
                        self.least_count += 1;
                        if self.usage.contains_key(&self.least_count) {
                            break;
                        }
                    }
                }
            },
            None => {},
        }
        
        if counters_is_empty {
            self.usage.remove(&existing_least_count);
        }
        
        // println!("POP {} {:?} {:?}", self.least_count, self.usage, self.data);
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if self.capacity < 1 {
            return;
        }
        
        let mut use_counter: i32 = 1;
        if let Some(x) = self.data.get_mut(&key) {
            use_counter = x.update(value);
        } else {
            if self.current_capacity >= self.capacity {
                self.pop_entry();
            }
            self.data.insert(key, Value{value: value, use_counter: 1});
            self.current_capacity += 1;
        }
        self.update_usage_counter(use_counter, key);
        // println!("PUT {} {:?} {:?}", self.least_count, self.usage, self.data);
    }
}

/**
 * Your LFUCache object will be instantiated and called as such:
 * let obj = LFUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

fn main() {
}