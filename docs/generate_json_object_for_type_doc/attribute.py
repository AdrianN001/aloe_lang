from code_block import CodeBlock

class Attribute:
    name: str
    description: str
    typeof: str
    example: CodeBlock

    def __init__(self, name: str, description: str, typeof: str, example: CodeBlock):
        self.name = name
        self.description = description
        self.typeof = typeof
        self.example = example