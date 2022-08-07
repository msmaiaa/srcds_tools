#[derive(Debug)]
pub enum Game {
    CSGO,
}

impl Game {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "csgo" => Some(Game::CSGO),
            _ => None,
        }
    }
    pub fn to_str(&self) -> &'static str {
        match self {
            Game::CSGO => "csgo",
        }
    }
    pub fn as_string_array() -> Vec<String> {
        vec!["csgo".to_string()]
    }
}

#[derive(Debug)]
pub struct ServerDirectory {
    pub game: Game,
    pub dir_root: String,
    pub dir_sourcemod: String,
    pub dir_sourcemod_configs: String,
    pub dir_sourcepython: String,
    pub dir_cfg: String,
    pub file_autoexec: String,
    pub file_servercfg: String,
}

impl ServerDirectory {
    pub fn new(dir: &str) -> Result<Self, String> {
        let found_game = find_game_from_root_dir(dir)?;
        let game_str = found_game.to_str();
        let game_dir = format!("{}\\{}", dir, game_str);
        Ok(Self {
            game: found_game,
            dir_root: dir.to_string(),
            dir_sourcemod: format!("{}\\addons\\sourcemod", game_dir),
            dir_sourcemod_configs: format!("{}\\addons\\sourcemod\\configs", game_dir),
            dir_sourcepython: format!("{}\\addons\\sourcepython", game_dir),
            dir_cfg: format!("{}\\cfg", game_dir),
            file_autoexec: format!("{}\\cfg\\autoexec.cfg", game_dir),
            file_servercfg: format!("{}\\cfg\\server.cfg", game_dir),
        })
    }
}

fn find_game_from_root_dir(dir: &str) -> Result<Game, String> {
    let mut dir_content = Vec::new();
    if let Ok(paths) = std::fs::read_dir(dir) {
        for entry in paths {
            dir_content.push(entry.unwrap().file_name().to_string_lossy().to_string());
        }
    } else {
        return Err("Invalid root directory".to_string());
    }

    let mut found: Option<Game> = None;
    for supported_game in Game::as_string_array() {
        if dir_content.contains(&supported_game) {
            found = Some(Game::from_str(&supported_game).unwrap());
        }
    }
    if found.is_none() {
        return Err("Invalid root directory, missing the game folder".to_string());
    }
    Ok(found.unwrap())
}
