from datafusion import SessionContext
import pyarrow as pa


ctx = SessionContext()

ctx.from_pylist([
    {'a': 1, 'b': 10.0, 'c': 'alpha'},
    {'a': 2, 'b': 20.0, 'c': 'beta'},
    {'a': 3, 'b': 30.0, 'c': 'gamma'},
]).show()

ctx.from_pydict({
    'a': [1, 2, 3],
    'b': [10.0, 20.0, 30.0],
    'c': ['alpha', 'beta', 'gamma'],
}).show()
