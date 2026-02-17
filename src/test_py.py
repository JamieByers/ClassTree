class Parent:
    def __init__(self) -> None:
        self.parent = None

class Pchild(Parent):
    def __init__(self) -> None:
        super().__init__()

class Child:
    def __init__(self) -> None:
        super().__init__()
