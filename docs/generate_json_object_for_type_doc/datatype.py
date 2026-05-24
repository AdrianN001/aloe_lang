from code_block import CodeBlock
from method import Method
from attribute import Attribute

class DataType:
    name: str
    description: str
    storage: str
    literal_syntax: CodeBlock

    def __init__(self, name: str, description: str, storage: str, example: CodeBlock) -> None:
        self.name = name
        self.description = description
        self.storage = storage
        self.literal_syntax = example

        self.methods = []
        self.attributes = []

    def add_method(self, method: Method) -> None:
        self.methods.append(method)
    
    def add_attribute(self, attribute: Attribute) -> None:
        self.attributes.append(attribute)


    def __repr__(self) -> str:
        return f"{self.name = }, number of attributes: {len(self.attributes)}, number of methods: {len(self.methods)}"