
#[derive(Debug)]
enum Media {
    Book { title: String, author: String},
    Movie { title: String, director: String},
    Audiobook { title: String},
}

impl Media {
    fn description(&self) -> String {
        if let Media::Audiobook { title } = self {
            format!("Audiobook: {}", title)
        } else if let Media::Book {title, author} = self {
            format!("Book {}, {}", title, author)
        } else if let Media::Movie {title, director} = self {
            format!("Movie: {}, {}", title, director)
        } else {
            String::from("Media description")
        }
    }
}
fn print_media(media: Media) {
    println!("{:#?}", media)
}

fn main() {
    let autiobook = Media::Audiobook { title: String::from("Awesome") };
    let goog_movie = Media::Movie { title: String::from("Good Movie"), director: String::from("Good Director") };
    let bad_book = Media::Book { title: String::from("Bad Book"), author: String::from("Bad Author") };

    println!("{}",autiobook.description());
    println!("{}",goog_movie.description());
    println!("{}",bad_book.description());

    println!("Hello, world!");
}
