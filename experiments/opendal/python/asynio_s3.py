import asyncio

async def main(bucket_name: str):
    op = opendal.AsyncOperator('s3', root='/tmp', bucket=bucket_name, region='ap-northeast-2')
    op.write('test.txt', b"Hello World")
    print(op.read('test.txt'))
    print(op.stat('test.txt').conet_length)
