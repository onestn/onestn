use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// `cat`의 러스트 버전
strcut Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// 줄 번호를 매긴다.
    #[arg(
        short('n'),
        long("number"),
        conflicts_with("number_nonblank_lines")
    )]
    number_lines: bool,

    /// 빈 줄이 아닐 때만 번호를 매긴다.
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}
