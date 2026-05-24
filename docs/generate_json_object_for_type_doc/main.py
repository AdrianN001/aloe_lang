
from code_block import CodeBlock
from datatype import DataType
from method import Method
from attribute import Attribute
import glob

def read_comments_of_the_file(file_path: str) -> list[str]:
    buffer = []
    currently_writing = False
    
    with open(file_path) as file:
        lines_raw = file.readlines()
        lines = (line.removeprefix("\t").removesuffix("\n") for line in lines_raw)

        comment_lines = (line for line in lines if line.startswith('#'))
        
        for comment_line in comment_lines:
            if comment_line.startswith('##'):
                if not currently_writing:
                    buffer.append([])
                    currently_writing = True
                else:
                    currently_writing = False
            buffer[-1].append(comment_line)
    
    return buffer

def parse_code_block(raw_comment_block: list[str]) -> CodeBlock:
    name = raw_comment_block[0].replace("#", "").replace("---", "")
    lines = [line.removeprefix("#") for line in raw_comment_block[1:-1]]
    lines_joined = "\n".join(lines)

    return CodeBlock(name, lines_joined)

def is_block_a_method(raw_comment_block: list[str]) -> bool:
    name = raw_comment_block[1]
    return "()" in name

def parse_datatype_block(raw_comment_block: list[str]) -> DataType:
    name = raw_comment_block[1].replace("# datatype: ", "")
    description = raw_comment_block[2].removeprefix("# ")
    storage = raw_comment_block[3].removesuffix("# ")
    literal_syntax = parse_code_block(raw_comment_block[6:])

    return DataType(name, description, storage, literal_syntax)

def parse_method_block(raw_comment_block: list[str]) -> Method:
    name = raw_comment_block[1].removeprefix("# ")
    description = raw_comment_block[2].removeprefix("# ")
    params = raw_comment_block[3].removeprefix("# params: [").removesuffix("]")
    returns = raw_comment_block[4].removeprefix("# returns: ")
    panics = raw_comment_block[5].removeprefix("# panics: [").removesuffix("]")
    errors = raw_comment_block[6].removeprefix("# errors: [").removesuffix("]")
    code_block = parse_code_block(raw_comment_block[9:])

    return Method(name, description, params, returns, panics, errors, code_block)

def parse_attribute_block(raw_comment_block: list[str]) -> Attribute:
    name = raw_comment_block[1].removeprefix("# ")
    description = raw_comment_block[2].removeprefix("# ")
    typeof = raw_comment_block[3].removeprefix("# type: ")

    example = parse_code_block(raw_comment_block[6:])

    return Attribute(name, description, typeof, example)

def get_doc_from_file(path: str) -> tuple[DataType, str]:
    raw_comment_blocks = read_comments_of_the_file(path)
 
    data_type = parse_datatype_block( raw_comment_blocks[0] )
    name_of_type = data_type.name

    for block in raw_comment_blocks[1:]:
        if is_block_a_method(block):
            method = parse_method_block(block)
            data_type.add_method(method)
        else:
            attribute = parse_attribute_block(block)
            data_type.add_attribute(attribute)

    return (data_type, name_of_type)

def main() -> None:
    all_doc_file = glob.glob("docs/types/*")

    for file_path in all_doc_file:
        name, doc = get_doc_from_file(file_path)
        print(name, doc)

if __name__ == "__main__":
    main()