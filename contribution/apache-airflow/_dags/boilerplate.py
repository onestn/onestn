from airflow.decorators import dag, task


@dag(dag_id='boilerplate')
def main():

    @task
    def test():
        print('test')

    test()

main()
