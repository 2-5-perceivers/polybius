use anyhow::Result;
use polybius::user_data::{NumType, Number, UserData};

fn main() -> Result<()> {
    println!("Welcome to Polybius, a smart password generator!!!");

    let mut data: UserData = UserData::default();

    // Mock up data until we have a way to get it from the user. For testing change the values to the ones you want to test.

    data.numbers_poll.push(Number::new(14, NumType::BirthDay));
    data.numbers_poll.push(Number::new(12, NumType::BirthDay));
    data.numbers_poll.push(Number::new(2005, NumType::BirthDay));
    data.numbers_poll
        .push(Number::new(2024, NumType::CurrentYear));

    data.text_poll.push("Apples".into());
    data.text_poll.push("Bananas".into());
    data.text_poll.push("Oranges".into());
    data.text_poll.push("Lemons".into());
    data.text_poll.push("Cats".into());
    data.text_poll.push("Dogs".into());
    data.text_poll.push("Lover".into());

    let p = data.generate_password(8);

    println!("Your password is: {}", p);

    Ok(())
}
