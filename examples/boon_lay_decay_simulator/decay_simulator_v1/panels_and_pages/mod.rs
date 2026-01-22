/// this basically allows the user to select the open panel
#[derive(serde::Deserialize, serde::Serialize,PartialEq,Clone)]
pub(crate) enum Panel {
    MainPage,
}


pub mod citation_disclaimer_and_acknowledgements;

pub mod main_page;
