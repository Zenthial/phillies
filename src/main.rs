use num_format::{Locale, ToFormattedString};
use scraper::{ElementRef, Html, Selector};
use ureq;

#[derive(Debug)]
enum Level {
    MLB,
    Other,
}

#[derive(Debug)]
struct Player {
    name: String,
    salary: i64,
    level: Level,
}

fn main() {
    let html = ureq::get("https://questionnaire-148920.appspot.com/swe/data.html")
        .call()
        .expect("Failed to get QO data")
        .into_string()
        .expect("Response returned no text");

    let doc = Html::parse_document(&html);
    let sel = Selector::parse("td").unwrap();

    let elements = doc.select(&sel).collect::<Vec<ElementRef>>();
    let mut players = Vec::new();

    for player_info in elements.chunks(4) {
        // for e in player_info {
        //     if e.value().attr("class").unwrap() == "player-name" {
        //         println!("{}", e.inner_html());
        //     }
        // }
        let name_ele = player_info[0];
        assert_eq!(name_ele.value().attr("class").unwrap(), "player-name");
        let player_name = name_ele.inner_html();

        let salary_ele = player_info[1];
        assert_eq!(salary_ele.value().attr("class").unwrap(), "player-salary");
        let player_salary = salary_ele.inner_html().trim_start_matches('$').to_string();
        let no_comma = player_salary.split(',').collect::<String>();
        // println!("{player_name}: {no_comma}");
        let actual_salary = no_comma.parse::<i64>().unwrap_or(0);

        let level_ele = player_info[3];
        assert_eq!(level_ele.value().attr("class").unwrap(), "player-level");
        let player_level = level_ele.inner_html();

        let player = Player {
            name: player_name,
            salary: actual_salary,
            level: if player_level == "MLB" {
                Level::MLB
            } else {
                Level::Other
            },
        };

        players.push(player);
    }

    players.sort_unstable_by(|a, b| b.salary.cmp(&a.salary));
    let qo = &players[..150].iter().map(|p| p.salary).sum() / 150 as i64;
    let offer = qo.to_formatted_string(&Locale::en);
    println!("The qualifying offer is {offer}");
}
