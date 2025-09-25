use clap::{Arg, ArgAction, Command};

pub fn get_command(){
    let matches = Command::new("bdc")
                        .about("Bdc gives the real time of dollar to naira")
                        .version("0.1.")
                        .subcommand(
                            Command::new("bdc")
                             .arg(
                            Arg::new("Rate")
                            .long("rate")
                            .action(ArgAction::SetTrue),
                             )).
                       get_matches();
 

    match  matches.subcommand() {
        Some(("bdc", rate_submatches)) =>{
            let value = rate_submatches.get_one::<bool>("Rate").unwrap();
            println!("{value}");
        },
    _ => print!("ERROR"),
    };

}
