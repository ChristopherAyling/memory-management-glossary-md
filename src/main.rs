use clap::Parser;
use html2md::parse_html;

fn make_url(letter: char) -> String {
    format!("https://www.memorymanagement.org/glossary/{}.html", letter)
}

fn download_letter(letter: char) -> String {
    let url = make_url(letter);
    ureq::get(url.as_str())
        .call()
        .expect("url call should work")
        .into_string()
        .expect("into string should work")
}

fn process_letter(letter: char, path: String) {
    let html = download_letter(letter);
    let md = parse_html(html.as_str());

    std::fs::write(path, md).expect("should be able to write to path");
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    output_folder: String,
}

fn main() {
    let args = Args::parse();
    let output_folder = args.output_folder;
    let missing_letters = vec![b'j', b'x', b'y'];

    for letter in b'a'..=b'z' {
        if missing_letters.contains(&letter) {
            continue;
        }
        let letter = letter as char;
        println!("letter: {}", letter as char);
        let letter_path = format!("{output_folder}/{letter}.md");
        process_letter(letter, letter_path);
    }
}
