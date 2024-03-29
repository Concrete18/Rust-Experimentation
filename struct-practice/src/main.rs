use chrono::{Local, NaiveDate};

struct Game {
    name: String,
    genre: String,
    release_date: Option<NaiveDate>,
}

impl Game {
    fn new(
        name: String,
        genre: String,
        release_year: i32,
        release_month: u32,
        release_day: u32,
    ) -> Self {
        let release_date = NaiveDate::from_ymd_opt(release_year, release_month, release_day);
        Self {
            name,
            genre,
            release_date,
        }
    }
}

impl Game {
    fn is_released(&self) -> bool {
        if let Some(release_date) = self.release_date {
            let current_date = Local::now().naive_local();
            release_date <= current_date.date()
        } else {
            false
        }
    }
}

fn main() {
    let game = Game::new("Hades".to_string(), "Action".to_string(), 2020, 8, 17);

    println!("{}", game.name);
    println!("{}", game.genre);
    // println!("{}", game.release_date);
    println!("{}", game.is_released());
}

#[cfg(test)]
mod struct_tests {
    use super::*;

    #[test]
    fn is_released_test() {
        let game = Game::new("Hades".to_string(), "Action".to_string(), 2020, 8, 17);
        assert!(game.is_released())
    }
}
