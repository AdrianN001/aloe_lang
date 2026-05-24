from code_block import CodeBlock

class Method:
    name: str
    description: str
    params: str
    returns: str
    panics: str
    errors: str
    example: CodeBlock

    def __init__(self, name: str, description: str, params: str, returns: str, panics: str, errors: str, example: CodeBlock):
        self.name = name
        self.description = description
        self.params = params
        self.returns = returns
        self.panics = panics
        self.errors = errors
        self.example = example 