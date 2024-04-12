import pycparser

ast = pycparser.parse_file("d.c", use_cpp=False)
print(ast)

