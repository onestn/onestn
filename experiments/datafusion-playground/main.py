import datafusion

ctx = datafusion.SessionContext()
df = ctx.sql("SELECT 1 + 1 AS two")

print(df.logical_plan())
