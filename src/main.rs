#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder
}

impl Media {
    fn description(&self) -> String {
        // if let Media::Audiobook { title } = self {
        //     format!("Audiobook: {}", title)
        // } else if let Media::Book {title, author} = self {
        //     format!("Book {}, {}", title, author)
        // } else if let Media::Movie {title, director} = self {
        //     format!("Movie: {}, {}", title, director)
        // } else {
        //     String::from("Media description")
        // }

        match self {
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Book { title, author } => {
                format!("Book {}, {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {}, {}", title, director)
            }
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder:")
            }
        }
    }
}

#[derive(Debug)]
struct  Catalog {
    items: Vec<Media>
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![]}
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }
}
fn print_media(media: Media) {
    println!("{:#?}", media)
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("Awesome"),
    };
    let goog_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    println!("{}", audiobook.description());
    println!("{}", goog_movie.description());
    println!("{}", bad_book.description());

    let mut catalog = Catalog::new();

    catalog.add(audiobook);
    catalog.add(goog_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    println!("{:#?}", catalog);

    println!("Hello, world!");
}
