import sys
from pycparser import parse_file, c_generator

def translate_to_c(filename):
    """ Simply use the c_generator module to emit a parsed AST.
    """
    ast = parse_file(filename, use_cpp=True)
    print(ast)
    generator = c_generator.CGenerator()
    print(generator.visit(ast))


if __name__ == "__main__":
    if len(sys.argv) > 1:
        translate_to_c(sys.argv[1])
    else:
        print("Please provide a filename as argument")
