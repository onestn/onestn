import polars as pl
from polars import col, lit, when

import datetime as dt


df = pl.DataFrame({
    'name': ['Alice Archer', 'Ben Brown', 'Chloe Cooper', 'Daniel Donova'],
    'birthdate': [
        dt.date(1997, 1, 10),
        dt.date(1985, 2, 15),
        dt.date(1983, 3, 22),
        dt.date(1981, 4, 30),
    ],
    'weight': [57.9, 72.5, 53.6, 83.1],
    'height': [1.56, 1.77, 1.65, 1.75]
})
print(df)

# df.write_csv('../data/output.csv')
# df_csv = pl.read_csv('../data/output.csv', try_parse_dates=True)

result = df.select(
    col('name'),
    col('birthdate').dt.year().alias('birth_year'),
    (col('weight') / (col('height') ** 2)).round(2).alias('bmi'),
)
print(result)

result = df.select(
    col('name'),
    (col('weight', 'height') * 0.95).round(2).name.suffix('-5%'),
)
print(result)

result = df.with_columns(
    birth_year=col('birthdate').dt.year(),
    bmi=col('weight') / (col('height') ** 2),
)
print(result)

result = df.filter(col('birthdate').dt.year() < 1990)
print(result)

result = df.filter(
    col('birthdate').is_between(dt.date(1982, 12, 31), dt.date(1996, 1, 1)),
    col('height') > 1.7,
)
print(result)

result = (df
    .group_by(
        (col('birthdate').dt.year() // 100).alias('decade'),
        maintain_order=True
    )
    .len()
)
print(result)

result = (df
    .group_by(
        (col('birthdate').dt.year() // 100).alias('decade'),
        maintain_order=True,
    )
    .agg(
        pl.len().alias('sample_size'),
        col('weight').mean().round(2).alias('avg_weight'),
        col('height').max().alias('tallest')
    )
)
print(result)

result = (df
    .with_columns(
        (col('birthdate').dt.year() // 100).alias('decade'),
        col('name').str.split(by=' ').list.first(),
    )
    .select(pl.all().exclude('birthdate'))
    .group_by(
        col('decade'),
        maintain_order=True
    )
    .agg(
        col('name'),
        col('weight', 'height').mean().round(2).name.prefix('avg_')
    )
)
print(result)

# [Combining dataframes]
df2 = pl.DataFrame({
    'name': ['Ben Brown', 'Daniel Donova', 'Alice Archer', 'Chloe Cooper'],
    'parent': [True, False, False, False],
    'siblings': [1, 2, 3, 4],
})
joined_df = df.join(df2, on='name', how='left')
print(joined_df)

# [Concatenating dataframes]
df3 = pl.DataFrame({
    'name': ['Ethan Edwards', 'Fiona Foster', 'Grace Gibson', 'Henry Harris'],
    'birthdate': [
        dt.date(1977, 5, 10),
        dt.date(1975, 6, 23),
        dt.date(1973, 7, 22),
        dt.date(1971, 8, 3),
    ],
    'weight': [67.9, 72.5, 57.6, 93.1],
    'height': [1.76, 1.6, 1.66, 1.8]
})
concated_df = pl.concat([df, df3], how='vertical')
print(concated_df)
