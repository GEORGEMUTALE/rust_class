#[derive(Clone)]
struct Player {
    name: String,
    shirt_number: i32,
}

struct Team {
    name: String,
    players: Vec<Player>,
}

trait TeamManagement {
    fn add_player(&mut self, player: Player);
    fn display_players(&self);
}

impl TeamManagement for Team {
    fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    fn display_players(&self) {
        println!("Team: {}", self.name);
        for player in &self.players {
            println!("{} wears shirt number {}", player.name, player.shirt_number);
        }
    }
}

fn main() {
    let mut team = Team {
        name: String::from("ECRI"),
        players: Vec::new(),
    };

    let player1 = Player {
        name: String::from("Arnold"),
        shirt_number: 7,
    };

    team.add_player(player1.clone());
    team.display_players();
}

// how the derive attribute is useful in enums and structs
