use opendal::services::S3;
use opendal::Operator;


fn main() {
    let mut builder = S3::default();
    builder.bucket("test");

    let op = Operator::new(builder)?
        .layer(LoggingLayer::default())
        .finish();

    let bs = op.read("hello.txt").await?;
}
