fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: s3deploy <spark|airflow> <filepath>");
        std::process::exit(1);
    }

    let target = &args[1];
    let filepath = &args[2];
    
    let s3_key = match target.as_str() {
        "spark" => "S3_EMR",
        "airflow" => "S3_MWAA",
        _ => {
            eprintln!("Invalid target: {}. Use 'spark' or 'airflow'", target);
            std::process::exit(1);
        }
    };
}
