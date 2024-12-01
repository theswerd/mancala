use crate::GamesResults;
use super::AlgorithmBattle;

use owo_colors::OwoColorize;

/*
use plotters::prelude::*;

pub fn mogus() {
    let mut root_area = BitMapBackend::new("mogus.png", (640, 480)).into_drawing_area();

    root_area.fill(&BLACK).unwrap();

    let style = TextStyle::from(("comic sans", 20).into_font()).color(&WHITE);

    ChartBuilder::on(&mut root_area)
        .x_label_area_size(20)
        .y_label_area_size(30)
        .margin_right(20)
        .caption("how about you chart some bitches", style)
        .build_cartesian_2d(-1f64..1f64, -1f64..1f64)
        .unwrap();

    root_area.present().unwrap();
}
*/

/*
pub fn bru<const S: usize>(results: Vec<AlgorithmBattle<S>>, algorithms: usize) {
    use term_grid::{Grid, GridOptions, Direction, Filling, Cell};

    let mut grid = Grid::new(GridOptions {
        filling: Filling::Text(" | ".to_string()),
        direction: Direction::LeftToRight,
    });
    
    grid.add(Cell::from("Second \\ First"));
    for second in 0..algorithms {
        grid.add(Cell::from(results[second * algorithms].first.name()));
    }

    for second in 0..algorithms {
        grid.add(Cell::from(results[second].second.name()));

        for first in 0..algorithms {
            let game_index = first + second * algorithms;
            let current_game = &results[game_index];

            let score = current_game.results.wins as isize - current_game.results.losses as isize;
            let score = score as f64 / current_game.results.games as f64;
            let draw_score = current_game.results.draws as f64 / current_game.results.games as f64;

            grid.add(Cell::from(format!("{} ({})", percent(score), percent(draw_score))));
        }
    }
    
    println!("{}", grid.fit_into_columns(algorithms + 1));
}
*/

pub fn draw_colors<const S: usize>(results: Vec<AlgorithmBattle<S>>, algorithms: usize) {
    const CHARACTER: char = '█'; // █ ▄

    const WIDTH: usize = 80;

    let max = (0..algorithms).into_iter().map(|v| results[v].second.name().len()).max().unwrap();

    let mut stuff: Vec<(String, usize)> = (0..algorithms).into_iter().map(|algoritm| {
        let as_first_games: GamesResults = (0..algorithms).into_iter().map(|first|  results[algoritm * algorithms + first].results).sum();
        let as_second_games:  GamesResults = (0..algorithms).into_iter().map(|second| results[second * algorithms + algoritm].results).sum();

        let complete = as_first_games + {
            let mut swapped = as_second_games;
            (swapped.wins, swapped.losses) = (as_second_games.losses, as_second_games.wins);
            (swapped.left_time, swapped.right_time) = (as_second_games.right_time, as_second_games.left_time);
            swapped
        };

        let name = results[algoritm].second.name();
        let total_width = WIDTH - max - 1;

        let wins = ((complete.wins as f64 / complete.games as f64) * total_width as f64) as usize;
        // let draws = ((complete.draws as f64 / complete.games as f64) * total_width as f64) as usize;
        let losses = ((complete.losses as f64 / complete.games as f64) * total_width as f64) as usize;
        let draws = total_width - (wins + losses);
        let time = complete.left_time;

        let character = CHARACTER.to_string();

        (format!(
            "{name}{} {}{}{} ({}/{}/{}) [{} / {}]",
            " ".repeat(max - name.len()),
            character.repeat(wins).green(),
            character.repeat(draws).blue(),
            character.repeat(losses).red(),
            complete.wins.green(), complete.draws.blue(), complete.losses.red(),
            format!("{:?}", time).bright_yellow(),
            format!("{:?}", time / complete.games as u32).bright_yellow(),
        ), wins)
    }).collect();

    stuff.sort_by(|a, b| a.1.cmp(&b.1));

    for i in (0..stuff.len()).rev() { println!("{}", stuff[i].0) }
}

/*
fn percent(percent: f64) -> String {
    let hundreds = (percent * 100.0).to_string();

    let percentage = if let Some((trunc, fract)) = hundreds.split_once(".") {
        format!("{}.{}", trunc, fract.chars().nth(0).unwrap())
    } else {
        hundreds
    };

    percentage + "%"
}
*/
