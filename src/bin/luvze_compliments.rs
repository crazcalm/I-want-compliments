use scraper::{Html, Selector};

fn get_html(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let html_bytes = reqwest::blocking::get(url)?.bytes()?;

    let data = std::str::from_utf8(&html_bytes)?;

    Ok(data.to_string())
}

fn compliment_does_not_include(compliment: &str, words: &Vec<&str>) -> bool {
    let mut result = true;

    for word in words {
        if compliment.contains(word) {
            result = false;
            break;
        }
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut compliments = Vec::new();
    let stop_words = vec![
        "wife",
        "women",
        "man",
        "husband",
        "guy",
        "girl",
        "marry",
        "kids",
        "kid",
        "child’s",
        "family",
        "mother",
        "father",
        "employee",
        "boss",
        "sister",
        "brother",
        "tattoo",
        "(",
        "swearing",
        "dad",
        "mom",
        "children",
    ];

    let urls = vec![
        (
            "https://www.luvze.com/compliments-for-men/",
            "h2:nth-of-type(2) ~ p",
        ),
        (
            "https://www.luvze.com/compliments-for-girls/",
            "h2:nth-of-type(1) ~ p",
        ),
    ];

    for (url, query_selector) in urls {
        let html_data = get_html(url)?;

        let document = Html::parse_document(&html_data);
        let selector = Selector::parse(query_selector).unwrap();

        for element in document.select(&selector) {
            let text = element.inner_html();
            if let Some((_, compliment)) = text.split_once(" ") {
                if compliment_does_not_include(&compliment.to_lowercase(), &stop_words) {
                    println!("{}", compliment);
                    compliments.push(compliment.to_string());
                }
            }
        }
    }

    println!("Total count: {}", compliments.len());

    Ok(())
}
