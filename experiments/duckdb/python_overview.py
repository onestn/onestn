"""
Example by: https://duckdb.org/docs/stable/clients/python/overview
"""
import duckdb


duckdb.sql("SELECT 42").show()

# [Basic API Usage]
# DuckDB can store relation to python variables
r1 = duckdb.sql("SELECT 42 AS i")
duckdb.sql("SELECT i * 2 AS k FROM r1").show()

# [Result Conversion]
duckdb.sql("SELECT 42").fetchall() # Python objects
duckdb.sql("SELECT 42").df()       # Pandas DataFrame
duckdb.sql("SELECT 42").pl()       # Polars DataFrame
duckdb.sql("SELECT 42").arrow()    # Arrow Table
duckdb.sql("SELECT 42").fetchnumpy # NumPy Arrays

# [Writing Data to Disk]
duckdb.sql("SELECT 42").write_parquet("out.parquet")
duckdb.sql("SELECT 42").write_csv("out.csv")
duckdb.sql("COPY (SELECT 42) TO 'out.parquet'")

# [Using an In-Memory Database]
con = duckdb.connect()
con.sql("SELECT 42 AS x").show()

# [Persistent Storage]
# Can use context manager
con = duckdb.connect('file.db')
con.sql("CREATE TABLE test (i INTEGER)")
con.sql("INSERT INTO test VALUES (42)")
con.table("test").show()
con.close()

with duckdb.connect('file.db') as con:
    con.sql("CREATE TABLE test (i INTEGER)")
    con.sql("INSERT INTO test VALUES (42)")
    con.table("test").show()

# [Configuration]
con = duckdb.connect(config={'threads': 1})

# [Connection Object and Module]
...

# [Community Extensions]
con = duckdb.connect()
con.install_extension('h3', repository='community')
con.load_extension('h3')
