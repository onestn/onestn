class Node:
    def __init__(self, data):
        self.data = data
        self.next = None

    def set_next(self, node):
        self.next = node


class LinkedList:
    def __init__(self):
        self.head = Node('head')
        self.tail = self.head

    @property
    def length(self):
        _node, length = self.head, 1

        while _node.next != None:
            _node = _node.next
            length += 1

        return length

    def append(self, value):
        _node = Node(value)
        self.tail.next = _node
        self.tail = _node
    
    # TODO: Add Exceptions
    def insert(self, n, value):
        new_node = Node(value)

        if n == 0:
            new_node.next = self.head
            self.head = new_node
            return

        current_node = self.head
        for _ in range(n - 1):
            current_node = current_node.next

        new_node.next = current_node.next
        current_node.next = new_node

    def select(self, n):
        if self.length - 1 < n or self.length < 0:
            raise Exception("Not accessable")
        
        if n == 0:
            return self.head.data
        
        current_node = self.head
        for _ in range(n):
            current_node = current_node.next

        return current_node.data
    
    def remove(self, n):
        current_node = self.head
        for _ in range(n - 1):
            current_node = current_node.next

        current_node.next = current_node.next.next

    def iterate_all_nodes(self):
        current_node = self.head
        for _ in range(self.length):
            print(current_node.data)
            current_node = current_node.next

linked_list = LinkedList()

linked_list.append('hello')
linked_list.append('world')
linked_list.append('tail')

linked_list.insert(1, 'some comma!')

linked_list.remove(3)

linked_list.iterate_all_nodes()