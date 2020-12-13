use ini::Ini;

pub struct Config { 
    pub trello: Trello
} 

pub struct Trello {
    pub key: String,
    pub token: String,
    pub board_id: String,
    pub list_id: String,
}

pub fn parse() -> Config {
    let mut conf_path = dirs::home_dir().unwrap();
    conf_path.push("my_conf.ini");

    let conf = Ini::load_from_file(conf_path).unwrap();

    let trello_section = conf.section(Some("Trello")).unwrap();
    let trello = Trello { 
        key: trello_section.get("key").unwrap().to_string(),
        token: trello_section.get("token").unwrap().to_string(),
        board_id: trello_section.get("board_id").unwrap().to_string(),
        list_id: trello_section.get("list_id").unwrap().to_string(),
    };

    Config { trello }
}
