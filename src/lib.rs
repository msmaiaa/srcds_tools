pub mod app_wrapper;
pub mod ui;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
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

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum OpenableKind {
    File,
    Folder,
}
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Openable {
    path: String,
    label: String,
    kind: OpenableKind,
}

impl Openable {
    pub fn new(path: String, label: &str, kind: OpenableKind) -> Self {
        Self {
            path,
            label: label.to_string(),
            kind,
        }
    }

    pub fn open(&self) -> Result<(), std::io::Error> {
        open::that(&self.path)
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ServerDirectory {
    pub game: Game,
    pub dir_root: Openable,
    pub dir_sourcemod: Openable,
    pub dir_sourcemod_configs: Openable,
    pub dir_sourcepython: Openable,
    pub dir_cfg: Openable,
    pub file_autoexec: Openable,
    pub file_servercfg: Openable,
    pub file_databases: Openable,
}

impl ServerDirectory {
    pub fn new(dir: &str) -> Result<Self, String> {
        use OpenableKind::*;
        let found_game = find_game_from_root_dir(dir)?;
        let game_str = found_game.to_str();
        let game_dir = format!("{}\\{}", dir, game_str);
        Ok(Self {
            game: found_game,
            dir_root: Openable::new(dir.to_string(), "Root", Folder),
            dir_sourcemod: Openable::new(
                format!("{}\\addons\\sourcemod", &game_dir),
                "SourceMod",
                Folder,
            ),
            dir_sourcemod_configs: Openable::new(
                format!("{}\\addons\\sourcemod\\configs", &game_dir),
                "Sourcemod configs",
                Folder,
            ),
            dir_sourcepython: Openable::new(
                format!("{}\\addons\\sourcepython", &game_dir),
                "SourcePython",
                Folder,
            ),
            dir_cfg: Openable::new(format!("{}\\cfg", &game_dir), "Cfg", Folder),
            file_autoexec: Openable::new(
                format!("{}\\cfg\\autoexec.cfg", &game_dir),
                "autoexec.cfg",
                File,
            ),
            file_servercfg: Openable::new(
                format!("{}\\cfg\\server.cfg", &game_dir),
                "server.cfg",
                File,
            ),
            file_databases: Openable::new(
                format!("{}\\addons\\sourcemod\\configs\\databases.cfg", &game_dir),
                "databases.cfg",
                File,
            ),
        })
    }
    pub fn openables(&self) -> Vec<&Openable> {
        vec![
            &self.dir_root,
            &self.dir_sourcemod,
            &self.dir_sourcemod_configs,
            &self.dir_sourcepython,
            &self.dir_cfg,
            &self.file_autoexec,
            &self.file_servercfg,
            &self.file_databases,
        ]
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
