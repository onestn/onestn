use polars::prelude::*;

fn main() -> PolarsResult<()> {
    let df = df![
        "name" => &["Alice", "Bob", "Charlie"],
        "age" => &[25, 30, 40]
    ]?;

    println!("{}", df);

    let filtered = df
        .lazy()
        .filter(col("age").gt(lit(30))
        .select([col("name"), col("age")])
        .collect()?;

    println!("Filtered:\n{}", filtered);

    Ok(())
}

