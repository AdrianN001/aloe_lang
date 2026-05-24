class CodeBlock:
    name: str
    source_code: str

    def __init__(self, name: str, source_code: str) -> None:
        self.name = name
        self.source_code = source_code

    
    def __repr__(self) -> str:
        return f"\n---{self.name}---\n{self.source_code}"