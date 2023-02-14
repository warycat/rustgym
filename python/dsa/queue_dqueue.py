# Import deque from collections module
from collections import deque

# Define a Queue class


class Queue:

    # Initialize an empty deque object
    def __init__(self):
        self.items = deque()

    # Check if the queue is empty
    def is_empty(self):
        return len(self.items) == 0

    # Add an item to the rear of the queue
    def push(self, item):
        self.items.append(item)

    # Remove and return an item from the front of the queue
    def pop(self):
        if self.is_empty():
            raise Exception("Queue is empty")
        return self.items.popleft()

    # Return the item at the front of the queue without removing it
    def peek(self):
        if self.is_empty():
            raise Exception("Queue is empty")
        return self.items[0]

    # Return the size of the queue
    def size(self):
        return len(self.items)

    def rotate_left_till_zero(self):
        while self.items[0] != 0:
            self.items.append(self.items.popleft())
        return self.items
            
if __name__ == "__main__":
    # Create a queue object
    q = Queue()

    # Check if the queue is empty
    print(q.is_empty())  # True

    # Add some items to the queue
    q.push(1)
    q.push(2)
    q.push(3)
    q.push(0)
    

    # Check the size of the queue
    print(q.size())  # 4

    # Peek the front item of the queue
    print(q.peek())  # 0
    
    print(q.rotate_left_till_zero())

    # Pop the front item of the queue
    print(q.pop())  # 0

    # Check the size of the queue
    print(q.size())  # 3

    # Pop the remaining items of the queue
    print(q.pop())  # 1
    print(q.pop())  # 2
    print(q.pop())  # 3

    # Check if the queue is empty
    print(q.is_empty())  # True

    # Try to pop from an empty queue
    # print(q.pop())  # Exception: Queue is empty
