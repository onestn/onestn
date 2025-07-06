import os

from pyiceberg.catalog import load_catalog
from pyiceberg.expressions import StartsWith

os.environ['PYICEBERG_HOME'] = os.getcwd()
catalog = load_catalog(name='local')
table = catalog.load_table('my_db.my_table')

result = table.scan(row_filter=StartsWith('name', 'metric')).to_arrow()

print(result.to_string(preview_cols=10))
