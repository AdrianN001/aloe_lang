from code_block import CodeBlock

class Attribute:
    name: str
    description: str
    example: CodeBlock

    def __init__(self, name: str, description: str, example: CodeBlock):
        self.name = name
        self.description = description
        self.example = example