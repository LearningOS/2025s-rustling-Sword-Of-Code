/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::new(),
			q2:Queue::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        if !self.q2.is_empty() {
            loop {
                if self.q2.is_empty() {
                    break;
                }
                self.q1.enqueue(self.q2.dequeue().expect("xxx"));
            }
        }
        self.q1.enqueue(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        if self.is_empty() {
            return Err("Stack is empty");
        }
        if self.q2.is_empty() {
            loop {
                if self.q1.size() == 1 {
                    break;
                }
                self.q2.enqueue(self.q1.dequeue().expect("xxxx"));
            }
            return self.q1.dequeue();
        } else {
            loop {
                if self.q2.size() == 1 {
                    break;
                }
                self.q1.enqueue(self.q2.dequeue().expect("xxxx"));
            }
            return self.q2.dequeue();
        }
    }

    pub fn is_empty(&self) -> bool {
		//TODO
        if !self.q1.is_empty() || !self.q2.is_empty() {
            return false;
        }
        
        return true;
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}