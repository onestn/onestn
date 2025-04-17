from airflow.decorators import dag, task


@dag(dag_id='test_provider_emr_serverless')
def main():
    pass


dag_instance = main()
