import opendal


bucket_name = 'deadline-datalake'

op = opendal.Operator('s3', root='/tmp', bucket=bucket_name, region='ap-northeast-2')

op.write('test.txt', b"Hello world!")

print(op.read('test.txt'))
print(op.stat('test.txt').content_length)
