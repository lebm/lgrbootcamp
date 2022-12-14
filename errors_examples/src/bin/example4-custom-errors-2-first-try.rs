use std::{collections::HashMap, error::Error, fmt::Display, io, num::ParseIntError};

fn main() {
    env_logger::init();

    let credit_cards = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 04 25"),
        ("Bob", "1234567 Dec 25 123"),
    ]);

    println!("Enter name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());

    match result {
        Ok(card) => println!("\nCredit Card Info: {card:?}"),
        Err(err) => {
            // Must use &err to avoid move of String contained in InvalidInput.
            // log::erro! macro below uses "err", so err must be valid.
            // Matching &err converts InvalidInput String in &String
            match &err {
                CreditCardError::InvalidInput(msg) => println!("{msg}"),
                CreditCardError::Other(_, _) => println!("\nSomething went wrong! Try again!"),
            }

            log::error!("\n{err:?}");
        }
    }
}

#[derive(Debug)]
enum CreditCardError {
    InvalidInput(String),
    Other(String, String),
}

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {
    let card_string = credit_cards
        .get(name)
        .ok_or(CreditCardError::InvalidInput(format!(
            "No credit card was found for {name}."
        )))?;

    let card = parse_card(card_string)
        .map_err(|e| CreditCardError::Other(e, format!("{name}'s card could not be parsed")))?;

    Ok(card)
}

#[derive(Debug)]
#[allow(dead_code)]
struct Expiration {
    year: u32,
    month: u32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Card {
    number: u32,
    exp: Expiration,
    cvv: u32,
}

fn parse_card(card: &str) -> Result<Card, String> {
    let mut numbers = parse_card_numbers(card).map_err(|e| e.to_string())?;

    let len = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(format!(
            "Incorrect number of elements parsed. Expect {expected_len} but get {len}. Elements: {numbers:?}"
        ));
    }

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Ok(Card {
        number,
        exp: Expiration { year, month },
        cvv,
    })
}

#[allow(dead_code)]
struct ParsePaymentInfoError {
    source: Option<Box<dyn Error>>,
    msg: Option<String>,
}

impl Display for ParsePaymentInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Parsing payment error: invalid payment info")
    }
}

// From trait converts from one type to another
// source is an optional owned reference (box) to a type implementation Error trait.
impl From<ParseIntError> for ParsePaymentInfoError {
    fn from(e: ParseIntError) -> Self {
        ParsePaymentInfoError {
            source: Some(Box::new(e)),
            msg: None,
        }
    }
}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card
        .split(" ")
        //.into_iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<u32>, _>>()?;

    Ok(numbers)
}
