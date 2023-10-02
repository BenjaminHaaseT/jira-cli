use std::rc::Rc;

mod models;
mod db;
use db::*;
mod ui;
mod io_utils;
use io_utils::*;
mod navigator;
use navigator::*;

fn main() {
    use clearscreen;

    let mut database = Rc::new(db::JiraDatabase::new("data/db.json".to_string()));
    let mut navigator = navigator::Navigator::new(database.clone());

    loop {
        clearscreen::clear().unwrap();
        // 1. get current page from navigator. If there is no current page exit the loop.
        let cur_page = match navigator.get_current_page() {
            Some(page) => page,
            None => break,
        };

        // 2. render page
        if let Err(e) = cur_page.draw_page() {
            eprintln!("error rendering page: {}\npress any key to continue...",e);
            wait_for_key_press();
        }

        // 3. get user input
        let user_input = get_user_input();

        // 4. pass input to page's input handler
        match cur_page.handle_input(&user_input) {
            Ok(Some(a)) => {
                if let Err(e) = navigator.handle_action(a) {
                    eprintln!("error performing action: {e}\npress any key to continue");
                    wait_for_key_press();
                }
            }
            Ok(None) => {
                eprintln!("error choosing action, press any key to continue");
                wait_for_key_press();
            }
            Err(e) => {
                eprintln!("error choosing action {}\npress any key to continue", e);
                wait_for_key_press();
            }
        }
        // 5. if the page's input handler returns an action let the navigator process the action
    }
}
