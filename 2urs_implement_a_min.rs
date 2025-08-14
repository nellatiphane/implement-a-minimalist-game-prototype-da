// 2urs_implement_a_min.rs

#[derive(Debug, Clone)]
struct Game {
    title: String,
    description: String,
    version: String,
}

#[derive(Debug, Clone)]
struct Level {
    id: u32,
    name: String,
    difficulty: u8,
}

#[derive(Debug, Clone)]
struct Player {
    id: u32,
    name: String,
    score: u32,
}

#[derive(Debug, Clone)]
struct Dashboard {
    game: Game,
    levels: Vec<Level>,
    players: Vec<Player>,
}

impl Dashboard {
    fn new(game: Game, levels: Vec<Level>, players: Vec<Player>) -> Self {
        Self { game, levels, players }
    }

    fn add_level(&mut self, level: Level) {
        self.levels.push(level);
    }

    fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    fn display_game_info(&self) {
        println!("Game Title: {}", self.game.title);
        println!("Game Description: {}", self.game.description);
        println!("Game Version: {}", self.game.version);
    }

    fn display_levels(&self) {
        println!("Levels:");
        for level in &self.levels {
            println!("ID: {}, Name: {}, Difficulty: {}", level.id, level.name, level.difficulty);
        }
    }

    fn display_players(&self) {
        println!("Players:");
        for player in &self.players {
            println!("ID: {}, Name: {}, Score: {}", player.id, player.name, player.score);
        }
    }
}

fn main() {
    let game = Game {
        title: "Minimalist Game".to_string(),
        description: "A simple game prototype".to_string(),
        version: "v0.1".to_string(),
    };

    let mut levels = Vec::new();
    levels.push(Level {
        id: 1,
        name: "Level 1".to_string(),
        difficulty: 1,
    });
    levels.push(Level {
        id: 2,
        name: "Level 2".to_string(),
        difficulty: 2,
    });

    let mut players = Vec::new();
    players.push(Player {
        id: 1,
        name: "Player 1".to_string(),
        score: 100,
    });
    players.push(Player {
        id: 2,
        name: "Player 2".to_string(),
        score: 200,
    });

    let mut dashboard = Dashboard::new(game, levels, players);

    dashboard.display_game_info();
    dashboard.display_levels();
    dashboard.display_players();

    dashboard.add_level(Level {
        id: 3,
        name: "Level 3".to_string(),
        difficulty: 3,
    });
    dashboard.add_player(Player {
        id: 3,
        name: "Player 3".to_string(),
        score: 300,
    });

    println!("\nUpdated Dashboard:");
    dashboard.display_game_info();
    dashboard.display_levels();
    dashboard.display_players();
}