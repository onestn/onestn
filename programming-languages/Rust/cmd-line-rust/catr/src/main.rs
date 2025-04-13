use clap::{Arg, ArgAction, Command};
use clap::Parser;

// #[derive(Debug)]
// struct Args {
//     files: Vec<String>,
//     number_lines: bool,
//     number_nonblank_lines: bool,
// }

// fn get_args() -> Args {
//     let matches = Command::new("catr")
//         .version("0.1.0")
//         .author("Ken Youens-Clark <kyclark@gmail.com)")
//         .about("Rust version of `cat`")
//         .arg(
//             Arg::new("files")
//                 .value_name("FILE")
//                 .help("Input file(s)")
//                 .num_args(1..)
//                 .default_value("-"),
//         )
//         .arg(
//             Arg::new("number")
//                 .short('n')
//                 .long("number")
//                 .help("Number lines")
//                 .action(ArgAction::SetTrue)
//                 .conflicts_with("number_nonblank"),
//         )
//         .arg(
//             Arg::new("number_nonblank")
//                 .short('b')
//                 .long("number-nonblank")
//                 .help("Number non-blank lines")
//                 .action(ArgAction::SetTrue),
//         )
//         .get_matches();
//     
//     Args {
//         files: matches.get_many("files").unwrap().cloned().collect(),
//         number_lines: matches.get_flag("number"),
//         number_nonblank_lines: matches.get_flag("number_nonblank"),
//     }
// }

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// `cat`의 러스트 버전
struct Args {
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

fn main() {
    // let args = get_args();
    let args = Args::parse();
    println!("{args:#?}");
}
