use std::error::Error;
use std::collections::HashSet;
use std::collections::HashMap;
// use std::time::Instant;

use csv::Reader;

use rand::Rng;
use rand::seq::SliceRandom;



#[derive(Debug, Clone)]
pub struct PlayerDetails {
    name: String, 
    base_mmr: f32,
    captain: bool,
    pos1_score: usize,
    pos2_score: usize,
    pos3_score: usize,
    pos4_score: usize,
    pos5_score: usize,
    willing_roles: Vec<usize>,

}

impl PlayerDetails {
    pub fn new(name: String, base_mmr: f32, captain: bool, 
            pos1_score: usize,
            pos2_score: usize,
            pos3_score: usize,
            pos4_score: usize,
            pos5_score: usize) -> PlayerDetails {

        
        let roles = vec![pos1_score, pos2_score, pos3_score, pos4_score, pos5_score];
        let mut willing_roles: Vec<usize> = vec![];

        for (index, &score) in roles.iter().enumerate() {
            if score > 0 {
                willing_roles.push(index + 1); 
            }
        }
        
        PlayerDetails { name, base_mmr, captain, pos1_score, pos2_score, pos3_score, pos4_score, pos5_score, willing_roles }
    }

    pub fn random_role(&self, available_roles: &mut Vec<usize>) -> Result<(usize, usize), Box<dyn Error>> {
        let mut rng = rand::rng();

        // Overlap

        let set1: HashSet<usize> = available_roles.iter().cloned().collect();
        let possible_roles: Vec<usize> = self.willing_roles.iter()
            .cloned()
            .filter(|x| set1.contains(x))
            .collect();

        if possible_roles.len() == 0{
            return Err("No available roles".into());
        }
            
        let index = rng.random_range(0..possible_roles.len());
        let num = possible_roles[index];

        available_roles.retain(|&x| x != num);

        match num {
            1 if self.pos1_score > 0 => Ok((1, self.pos1_score)), 
            2 if self.pos2_score > 0 => Ok((2, self.pos2_score)),
            3 if self.pos3_score > 0 => Ok((3, self.pos3_score)),
            4 if self.pos4_score > 0 => Ok((4, self.pos4_score)),
            5 if self.pos5_score > 0 => Ok((5, self.pos5_score)),
            _ => Err("Unexpected error".into()),
        }
    }
}

pub fn load_players_from_csv() -> Result<Vec<PlayerDetails>, Box<dyn Error>> {
    let mut rdr = Reader::from_path("/app/data/input.csv")?;
    let mut players: Vec<PlayerDetails> = vec![];

    for result in rdr.records() {
        let record = result?;

        let name = record[0].to_string();

        let player = PlayerDetails::new(record[0].to_string(), 
                                        record[1].parse::<f32>().expect(&format!("Error parsing MMR f32 for player {}", name)), 
                                        record[2].to_lowercase().parse::<bool>().expect(&format!("Error parsing Captain bool for player {}", name)), 
                                        record[3].parse::<usize>().expect(&format!("Error parsing Pos1 preference for player {}", name)), 
                                        record[4].parse::<usize>().expect(&format!("Error parsing Pos2 preference for player {}", name)), 
                                        record[5].parse::<usize>().expect(&format!("Error parsing Pos3 preference for player {}", name)), 
                                        record[6].parse::<usize>().expect(&format!("Error parsing Pos4 preference for player {}", name)), 
                                        record[7].parse::<usize>().expect(&format!("Error parsing Pos5 preference for player {}", name)));
        players.push(player);

    }

    match players.len() {
        10 => Ok(players),
        _ => Err("Unexpected".into()),
    }

}


pub type TeamMap = HashMap<usize, (usize, String, f32, bool)>;

fn handle_team(players: &[PlayerDetails]) -> Result<TeamMap, Box<dyn Error>> {
    let players: Vec<PlayerDetails> = players.to_vec();
    
    let mut available_roles: Vec<usize> = vec![1, 2, 3, 4, 5];

    // HashMap where key = position, value = (score, name)
    let mut team_map = HashMap::new();
    

    for player in players {
        let (position, score) = player.random_role(&mut available_roles)?;
        team_map.insert(position, (score, player.name, player.base_mmr, player.captain));
        
    }
    
    Ok(team_map)

}

fn try_handle_team(input: &[PlayerDetails]) -> Result<TeamMap, Box<dyn std::error::Error>> {
    let max_attempts = 10;
    for _ in 1..=max_attempts {
        match handle_team(input) {
            Ok(team) => return Ok(team),
            Err(_e) => {}
        }
    }
    Err(format!("Failed to handle team after {} attempts", max_attempts).into())
}


fn generate_scenario(players: &mut Vec<PlayerDetails>, best_score: &mut f32, 
                        best_solutions: &mut Solutions) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::rng();
    players.shuffle(&mut rng);

    let (team_one, team_two) = players.split_at(5);

    let mut team_one: TeamMap = try_handle_team(team_one)?;
    let mut team_two: TeamMap = try_handle_team(team_two)?;
    
    let score: f32 = evaluate_solution(&mut team_one, &mut team_two);

    if score < *best_score {
        *best_score = score.clone();
        best_solutions.insert(0, (score, team_one, team_two));
    };

    Ok(())
}

fn discount_mmr_on_comfort(team: &mut TeamMap) {
    for (key, value) in team.iter_mut() {
        if *key == 2 {
            value.2 *= 0.95_f32.powf(10.0 - value.0 as f32);
        } else {
            value.2 *= 0.965_f32.powf(10.0 - value.0 as f32);
        }
    }
}

fn evaluate_solution(team_one: &mut TeamMap, team_two: &mut TeamMap) -> f32 {
    // Score 
    let mut score: f32 = 0.0;

    // Discounting MMR based off role comfort
    discount_mmr_on_comfort(team_one);
    discount_mmr_on_comfort(team_two);

    // Overall MMR diff
    let team_one_total_mmr: f32 = team_one.values().map(|value| value.2).sum();
    let team_two_total_mmr: f32 = team_two.values().map(|value| value.2).sum();

    let total_mmr_dff: f32 = team_one_total_mmr-team_two_total_mmr;

    score += (total_mmr_dff.abs().powf(1.15))/1000.0;


    // Mid MMR diff
    let mid_mmr_diff: f32 = (team_one[&2].2 - team_two[&2].2).abs();
    
    score += mid_mmr_diff.powf(1.3)/1500.0;


    // Sidelane MMR diff
    let onelane_mmr_diff: f32 = ((team_one[&1].2+team_one[&5].2)-(team_two[&3].2+team_two[&4].2)).abs();
    let otherlane_mmr_diff: f32 = ((team_one[&3].2+team_one[&4].2)-(team_two[&1].2+team_two[&5].2)).abs();

    score += onelane_mmr_diff.powf(1.2)/1500.0;
    score += otherlane_mmr_diff.powf(1.2)/1500.0;


    // Player satisfaction
    for i in 1..=5 {
        score += 10.0-team_one[&i].0 as f32;
        score += 10.0-team_two[&i].0 as f32;
    }

    // Captain present

    let team_one_captain = team_one.values().any(|&(_, _, _, flag)| flag);
    if !team_one_captain {
        score += 3.0;
    }

    let team_two_captain = team_two.values().any(|&(_, _, _, flag)| flag);
    if !team_two_captain {
        score += 3.0;
    }

    score
}

pub type Solutions = Vec<(f32, TeamMap, TeamMap)>;
pub fn find_best_solutions(players: &mut Vec<PlayerDetails>, n: usize) -> Solutions {
    let mut best_score: f32 = f32::INFINITY;
    let mut best_solutions: Solutions = vec![];

    for _ in 0..n {
        if let Err(_e) = generate_scenario(players, &mut best_score, &mut best_solutions) {
            continue;
        }
    }

    best_solutions
}



pub fn run_matchmaker(n: usize) -> Solutions {
    assert!(n > 0);

    let mut players = load_players_from_csv().unwrap();
 
    let solutions: Solutions = find_best_solutions(&mut players, n);    

    solutions
}