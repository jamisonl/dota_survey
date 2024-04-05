use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Subcategory {
    #[serde(rename = "type")]
    type_name: String,
    characteristics: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Category {
    carry: Vec<Subcategory>,
    offlane: Vec<Subcategory>,
    support: Vec<Subcategory>,
}

fn ask_questions(questions: &[String]) -> bool {
    let mut true_count = 0;

    for question in questions {
        print!("{}\nRespond with 1 for true, 2 for false: ", question);
        io::stdout().flush().unwrap();
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();
        if answer.trim() == "1" {
            true_count += 1;
        }

        if (true_count as f64 / questions.len() as f64) >= 0.65 {
            return true;
        }
    }

    false
}

fn evaluate_type(subcategories: &[Subcategory], _category_key: &str) -> Option<String> {
    for subcategory in subcategories {
        println!("Evaluating: {}", subcategory.type_name);
        if ask_questions(&subcategory.characteristics) {
            return Some(subcategory.type_name.clone());
        }
    }

    None
}

fn select_category(categories: &HashMap<String, Vec<Subcategory>>) -> Option<String> {
    println!("Select a category (carry, offlane, support): ");
    let mut category_name = String::new();
    io::stdin().read_line(&mut category_name).unwrap();
    let category_name = category_name.trim();

    if let Some(subcategories) = categories.get(category_name) {
        if let Some(type_name) = evaluate_type(subcategories, category_name) {
            return Some(format!("{} {}", type_name, category_name));
        } else {
            println!("You defy all expectations, you are the 'Jopa'");
        }
    } else {
        println!("Invalid category. Please select again.");
    }

    None
}

fn main() {
    let json_data = include_str!("questions.json");
    let categories: HashMap<String, Vec<Subcategory>> =
        serde_json::from_str(&json_data).expect("Error parsing JSON");

    while let Some(result) = select_category(&categories) {
        println!("Result: You are the \"{}\".", result);
        break;
    }

    println!("Do you want to continue with a new category? (yes/no)");
    let mut decision = String::new();
    io::stdin().read_line(&mut decision).unwrap();
    if decision.trim().eq_ignore_ascii_case("yes") {
        main();
    }
}
