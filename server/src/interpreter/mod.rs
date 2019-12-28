#![allow(unused)]
mod error;
pub mod parse;

use crate::currency::*;
use crate::transaction::*;
use parse::*;
use ::chrono::*;
use crate::person::*;

use std::io::{
    self,
    Read,
    Write,
};

impl<'a> Parse<'a> for Transaction<Euro> {
    named!(parse(&'a str) -> Self,
    map!(
        tuple!(
            opt!(
                preceded!(
                    space0,
                    DateTime::<Utc>::parse
                    )),
            preceded!(
                space0,
                Subject::parse
                ),
            preceded!(
                space1,
                Action::parse
                ),
            preceded!(
                space1,
                Euro::parse
                ),
            opt!(
                preceded!(
                    space1,
                    Subject::parse
                    ))
            ),
        |(date, sender, action, amount, recipent)|
                    Transaction::default()
                        .set_date(match date {
                            Some(d) => d,
                            None => Utc::now(),
                        })
                    .set_amount(match action {
                        Action::Get => amount,
                        Action::Give => -amount
                    })
                    .clone()
    )
        );
}
pub fn run() -> io::Result<()> {
    println!("Running interpreter ");
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.as_str() {
            "q" | "quit" | "exit" | ":q" => break,
            _ => println!("{:?}", Transaction::<Euro>::parse(&input)),
        };
    }
    Ok(())
}