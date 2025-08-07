mod content;
use content::catalog::Catalog;
use content::media::Media;
// enum MightHaveAValue<'a> {
//     ThereIsAValue(&'a Media),
//     NoValueAvailable
// }

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

    // this enforce us to handle the case in which we have a value and the case in which didn't
    println!("{:#?}",catalog.items.get(0)); //Here we will get the value wrap inside the some()
    println!("{:#?}",catalog.items.get(100)); // there is no item existe in this case we will get None

    // match catalog.items.get(0) {
    //     Option::Some(value) => {
    //         println!("Item: {:#?}", value)
    //     }
    //     Option::None => {
    //         println!("Nothing at that index")
    //     }
    // }

    match  catalog.get_by_index(10) {
        Some(value) => {
            println!("{:#?}",  value)
        }
        None => {
            println!("Nothing is at that index")
        }
    }

    let item= catalog.get_by_index(1);
    item.unwrap(); // unwrap the value from some
    item.expect("Expected to be a item a here"); // if there  a none here instead of panic we will get this message

    println!("Hello, world!");
}
