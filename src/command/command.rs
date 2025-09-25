use clap::{Arg, ArgAction, Command};

use crate::request::request::get_usdt_ngn_rate;
pub async  fn start_bdc() -> Result<(),  Box<dyn std::error::Error>>{
    let matches = Command::new("bdc")
                        .about("Bdc gives the real time of dollar to naira")
                        .version("0.1.")
                        .subcommand(
                            Command::new("rate")
                             .arg(
                            Arg::new("Rate")
                            .long("rate")
                            .action(ArgAction::SetTrue),
                             )).
                       get_matches();
 

    match  matches.subcommand() {
        Some(("rate", rate_submatches)) =>{
            let _ = rate_submatches.get_one::<bool>("Rate").unwrap();
            let result = get_usdt_ngn_rate().await;
            let value = result.ok().unwrap();
            println!("1 USDT == {value} NGN");
        },
    _ => print!("ERROR"),
    };

    Ok(())

}
