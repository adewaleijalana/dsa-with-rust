pub struct CustomArray<T> {
    data: Vec<T>,
    length: usize,
}

impl<T> CustomArray<T> {
    pub fn new() -> Self {
        CustomArray {
            data: Vec::new(),
            length: 0,
        }
    }

    pub fn add(&mut self, item: T) {
        self.data.push(item);
        self.length += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.length {
            Some(&self.data[index])
        } else {
            None
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.length {
            let item = self.data.remove(index);
            self.length -= 1;
            Some(item)
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.length > 0 {
            let item = self.data.pop();
            self.length -= 1;
            item
        } else {
            None
        }
    }

    pub fn length(&mut self) -> usize {
        self.length
    }
}