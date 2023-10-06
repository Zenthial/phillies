use num_format::{Locale, ToFormattedString};
use scraper::{ElementRef, Html, Selector};
use ureq;

const TOP_PLAYERS: usize = 125;

// Enum to differentiate professional level
// Level should always be MLB
enum Level {
    MLB,
    Other,
}

// Data type for an individual player
// Only salary is currently used, the dead code macro disables compile warnings
#[allow(dead_code)]
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

    let elements: Vec<ElementRef> = doc.select(&sel).collect();
    let mut players = Vec::new();

    // iterate through chunks of four, as the html is always structured in the form:
    // <td class='player-name'></td>
    // <td class='player-salary'></td>
    // <td class='player-year'></td>
    // <td class='player-level'></td>
    for player_info in elements.chunks(4) {
        let name_ele = player_info[0];
        assert_eq!(name_ele.value().attr("class").unwrap(), "player-name");
        let player_name = name_ele.inner_html();

        let salary_ele = player_info[1];
        assert_eq!(salary_ele.value().attr("class").unwrap(), "player-salary");
        let player_salary = salary_ele.inner_html().trim_start_matches('$').to_string();
        let no_comma: String = player_salary.split(',').collect();
        let actual_salary: i64 = no_comma.parse().unwrap_or(0);

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

    // We can sort unstable because we do not care about perserving order of equal elements
    // Two different players with the same salary switching places doesn't affect the average
    players.sort_unstable_by(|a, b| b.salary.cmp(&a.salary));

    let top = &players[..TOP_PLAYERS];
    // Iterate through the highest paid players, mapping the Iter<Player> into an Iter<i64>
    // Then sum that Iter<i64>, dividing that sum by the amount of highest paid players
    let qo = top.iter().map(|p| p.salary).sum::<i64>() / (TOP_PLAYERS as i64);

    // Format the qualifying offer number into a presentable format
    let offer = qo.to_formatted_string(&Locale::en);
    println!("The qualifying offer value is ${offer}");
}
