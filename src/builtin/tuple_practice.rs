pub fn tuple_practice(){
    let coord = (2, 3);
    println!("{:?}, {:?}", coord.0, coord.1);

    let (x, y) = (2, 3);
    println!("{:?}, {:?}", x, y);

    let user_info = ("Dong-Gil", 31);
    println!("{:?}, {:?}", user_info.0, user_info.1);

    let favorites = (
        "Game",
        "Dev",
        "rust",
        "java",
        "go",
        "spring",
        "js",
        "typescript",
    );
    println!(
        "{:?}, {:?}, {:?} {:?}",
        favorites.0, favorites.2, favorites.4, favorites.7
    );
}