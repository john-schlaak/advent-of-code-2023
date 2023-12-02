pub fn sum_valid_game_ids(games: String, red_limit: u32, green_limit: u32, blue_limit: u32) -> u32 {
    games.split('\n').map(
        |game| get_id_from_game_if_valid(game.trim(), red_limit, green_limit, blue_limit)
    ).sum()
}


fn get_id_from_game_if_valid(game: &str, red_limit: u32, green_limit: u32, blue_limit: u32) -> u32 {
    let colon_split = game.split(':').collect::<Vec<&str>>();
    if let (Some(id_term), Some(grabs_term)) = (colon_split.get(0), colon_split.get(1)) {
        let space_split = id_term.split_whitespace().collect::<Vec<&str>>();
        let id = if let Some(id_str) = space_split.get(1) {
            id_str.parse::<u32>().unwrap_or(0)
        } else {
            0
        };
        let valid = {
            let grab_terms = grabs_term.split(';').collect::<Vec<&str>>();
            grab_terms.iter().all(
                |grab_term| {
                    let color_terms = grab_term.split(',').map(|color_term| color_term.trim()).collect::<Vec<&str>>();
                    color_terms.iter().all(
                        |color_term| {
                            let space_split = color_term.split_whitespace().collect::<Vec<&str>>();
                            if let (Some(count_str), Some(color)) = (space_split.get(0), space_split.get(1)) {
                                let count = count_str.parse::<u32>().unwrap_or(0);
                                match *color {
                                    "red" => count <= red_limit,
                                    "green" => count <= green_limit,
                                    "blue" => count <= blue_limit,
                                    _ => false
                                }
                            } else {
                                false
                            }
                        }
                    )
                }
            )
        };
        if valid {
            return id;
        }
    }
    return 0;
}