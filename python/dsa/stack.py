"""Stack data structure implementation in Python with the following methods:
    - push
    - pop
    - peek
    - is_empty
    - size
    - reverse

    Raises:
        Exception: If the stack is empty
        Exception: If the stack is empty

    Returns:
        Stack: Description of returned object.
"""


class Stack:

    # Define the constructor with an empty list as the attribute
    def __init__(self):
        self.items = []

    # Define a method to push an element to the stack
    def push(self, item):
        self.items.append(item)

    # Define a method to pop an element from the stack
    def pop(self):
        if self.is_empty():
            raise Exception("Stack is empty")
        return self.items.pop()

    # Define a method to peek at the top element of the stack
    def peek(self):
        if self.is_empty():
            raise Exception("Stack is empty")
        return self.items[-1]

    # Define a method to check if the stack is empty
    def is_empty(self):
        return len(self.items) == 0

    # Define a method to get the size of the stack
    def size(self):
        return len(self.items)

   # Define a method to reverse the stack using recursion
    def reverse(self):
        # Print the start of the reverse function
        print("Start of reverse function")
        # Print the stack
        print("Stack:", self.items)
        # Base case: if the stack is empty, do nothing
        if self.is_empty():
            # Print the end of the reverse function
            print("End of reverse function based on empty stack")
            return
        # Recursive case: pop the top element and store it
        item = self.pop()
        # Print the item
        print("Item:", item)
        # Reverse the remaining stack
        self.reverse()
        # Insert the popped element at the bottom of the stack
        self.insert_at_bottom(item)
        # Print the end of the reverse function
        print("Finally End of reverse function")

    # Define a helper method to insert an element at the bottom of the stack
    def insert_at_bottom(self, item):
        # Print the start of the insert_at_bottom function
        print("Start of insert_at_bottom function")
        # Print the stack
        print("Stack:", self.items)
        # Print the item
        print("Item:", item)
        # Base case: if the stack is empty, push the item
        if self.is_empty():
            self.push(item)
            # Print the end of the insert_at_bottom function
            print("End of insert_at_bottom function based on empty stack")
            return
        # Recursive case: pop the top element and store it
        else:
            temp = self.pop()
            # Print the temp
            print("Temp:", temp)
            # Insert the item at the bottom of the remaining stack
            self.insert_at_bottom(item)
            # Push the popped element back to the stack
            self.push(temp)
            # Print the end of the insert_at_bottom function
            print("End of insert_at_bottom function")


# Create an object of the Stack class
stack = Stack()

# Push elements to the stack
stack.push(1)
stack.push(2)
stack.push(3)

# Print the stack
print(stack.items)

# Reverse the stack
stack.reverse()

# Print the stack
print(stack.items)
