mod command;
mod request;
use crate::command::command::start_bdc;

#[tokio::main]
async  fn main() {
    let _ = start_bdc().await;
}
