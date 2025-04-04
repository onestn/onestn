use polars::preludes::*;

let q = LazyCsvReader::new("docs/assets/data/iris.csv")
    .with_has_header(true)
    .finish()?
    .filter(col("sepal_length").gt(lit(5)))
    .group_by(vec![col("species")])
    .agg([col("*").sum()]);

let df = q.collect()?;
