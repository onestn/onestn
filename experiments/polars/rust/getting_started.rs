use chrono::prelude::*;
use polars::prelude::*;

let mut df: DataFrame = df!(
    "name" => ["Alice Archer", "Ben Brown", "Chloe Cooper", "Daniel Donovan"],
    "birthdate" => [
        NaiveDate::from_ymd_opt(1997, 1, 10).unwrap(),
        NaiveDate::from_ymd_opt(1985, 2, 15).unwrap(),
        NaiveDate::from_ymd_opt(1983, 3, 22).unwrap(),
        NaiveDate::from_ymd_opt(1981, 4, 30).unwrap(),
    ],
    "weight" => [57.9, 72.5, 53.6, 83.1],
    "height" => [1.56, 1.77, 1.65, 1.75],
)
.unwrap();
println!("{df}");

use std::fs::File;

let mut file = File::create("../data/output.csv").expect("could not create file");
CsvWriter::new(&mut file)
    .include_header(true)
    .with_seperator(b',')
    .finish(&mut df)?;

let df_csv = CsvReadOptions::default()
    .with_has_header(true)
    .with_parse_options(CsvParseOptions::default().with_try_parse_dates(true))
    .try_into_reader_with_file_path(Some("../data/output.csv".into()))?
    .finish()?;
println!("{df_csv}");

// [select]
let result = df
    .clone()
    .lazy()
    .select([
        col("name"),
        col("birthdate").dt().year().alias("birth_year"),
        (col("weight") / col("height").pow(2)).alias("bmi"),
    ])
    .collect()?;
println!("{result}");

let result = df.clone().lazy()
    .select([
        col("name"),
        (cols(["weight", "height"]).as_expr() * lit(0.95))
            .round(2, RoundMode::default())
            .name()
            .suffix("-5%"),
    ])
    .collect()?;

println!("{result}");
