use std::cmp::max;

struct Game {
    id: u32,
    grabs: Vec<Grab>
}

impl Game {
    fn is_valid(&self, rgb_limits: (u32, u32, u32)) -> bool {
        self.grabs.iter().all(
            |grab| [(grab.red, rgb_limits.0), (grab.green, rgb_limits.1), (grab.blue, rgb_limits.2)].iter().all(
                |(grab_color, limit)| grab_color.is_none() || grab_color.unwrap() <= *limit
            )
        )
    }

    fn get_minimum_cubes_grab(&self) -> Grab {
        let [red, green, blue] = self.grabs.iter().map(
            |grab| [grab.red, grab.green, grab.blue]
        ).reduce(
            |min_colors, colors| {
                let result = min_colors.into_iter().zip(colors.into_iter()).map(
                    |(min_color, color)| if let (Some(min_color), Some(color)) = (min_color, color) {
                        Some(max(min_color, color))
                    } else if min_color.is_none() {
                        color
                    } else {
                        min_color
                    }
                ).collect::<Vec<Option<u32>>>();
                [result[0], result[1], result[2]]
            }
        ).unwrap_or([None, None, None]);
        Grab {red, green, blue}
    }
}

struct Grab {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>
}

impl Grab {
    fn get_power(&self) -> u32 {
        self.red.unwrap_or(1) * self.green.unwrap_or(1) * self.blue.unwrap_or(1)
    }
}

enum Color {
    Red(u32),
    Green(u32),
    Blue(u32)
}

pub fn sum_valid_game_ids(games_string: String, rgb_limits: (u32, u32, u32)) -> u32 {
    parse_games(games_string).iter().map(|game| if game.is_valid(rgb_limits) { game.id } else { 0 }).sum()
}


pub fn sum_powers_of_minimum_grabs(games_string: String) -> u32 {
    parse_games(games_string).iter().map(|game| game.get_minimum_cubes_grab().get_power()).sum()
}


fn parse_games(games_string: String) -> Vec<Game> {
    games_string.split('\n').map(
        |game_string| {
            if let Some([id_term, grabs_term]) = game_string.trim().split(':').collect::<Vec<&str>>().get(0..2) {
                let id = if let Some(id_str) = id_term.split_whitespace().collect::<Vec<&str>>().get(1) {
                    if let Ok(id) = id_str.parse::<u32>() {
                        id
                    } else {
                        panic!("Could not parse id for game '{}'", game_string)
                    }
                } else {
                    panic!("Could not parse id_str for game '{}'", game_string)
                };
                let grabs = grabs_term.split(';').map(
                    |colors_term| {
                        let colors = colors_term.trim().split(',').map(
                            |color_term| if let Some([count_str, color_str]) = color_term.split_whitespace().collect::<Vec<&str>>().get(0..2) {
                                let count = if let Ok(count) = count_str.parse::<u32>() {
                                    count
                                } else {
                                    panic!("Could not interpret a count_str as a number for game '{}'", game_string)
                                };
                                match *color_str {
                                    "red" => Color::Red(count),
                                    "green" => Color::Green(count),
                                    "blue" => Color::Blue(count),
                                    _ => panic!("Could not interpret a color_str as 'red', 'green', or 'blue' for game '{}'", game_string)
                                }
                            } else {
                                panic!("Could not parse a color_term for game '{}'", game_string)
                            }
                        );
                        let mut grab = Grab {red: None, green: None, blue: None};
                        for color in colors {
                            match color {
                                Color::Red(count) => grab.red = Some(grab.red.unwrap_or(0) + count),
                                Color::Green(count) => grab.green = Some(grab.green.unwrap_or(0) + count),
                                Color::Blue(count) => grab.blue = Some(grab.blue.unwrap_or(0) + count)
                            }
                        }
                        grab
                    }
                ).collect::<Vec<Grab>>();
                Game{id, grabs}
            } else {
                panic!("Could not parse id_term and grabs_term for game '{}'", game_string)
            }
        }
    ).collect::<Vec<Game>>()
}