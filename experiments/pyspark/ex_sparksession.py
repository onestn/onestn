from pyspark.sql import SparkSession

# SparkSession 초기화
spark = (SparkSession.builder
    .appName("LocalSparkSessionExample")
    .master("local[*]")
    .getOrCreate()
)

print("Spark Version:", spark.version)

spark.stop()