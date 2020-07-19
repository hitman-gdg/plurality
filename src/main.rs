use std::io::{stdin, stdout, Write};

const MAX: usize = 9;

struct Candidate {
    name: String,
    votes: u8
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env::args;
    use std::process::exit;

    if args().len() < 2 { exit(1); }
    let candidate_count = args().len() - 1;
    if candidate_count > MAX { exit(2); }
    let mut candidates: Vec<Candidate> = Vec::with_capacity(MAX as usize);
    for i in 0..candidate_count {
        candidates.push(Candidate {
            name: args().nth(i + 1).unwrap(),
            votes: 0
        });
    }
    let voter_count = read_string("Number of voters: ").parse()?;   
    for _ in 0..voter_count {
        let name = read_string("Vote: ");
        if !vote(&name, candidate_count as usize, &mut candidates) { println!("Invalid vote."); }
    }
    print_winner(candidate_count as usize, &candidates);
    Ok(())
}

fn read_string(input_text: &'static str) -> String {
    let mut s = String::new();
    print!("{}", input_text);
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Not a correct string");
    while let Some('\n') | Some('\r') = s.chars().next_back() { s.pop(); }
    s
}

fn vote(name: &str, count: usize, candidates: &mut Vec<Candidate>) -> bool {
    for i in 0..count {
        if candidates[i].name == name {
            candidates[i].votes += 1;
            return true;
        }
    }
    false
}

fn print_winner(count: usize, candidates: &[Candidate]) {
    let mut max_votes = 0u8;
    for i in 0..count {
        if candidates[i].votes > max_votes {
            max_votes = candidates[i].votes;
        }
    }
    for i in 0..count {
        if candidates[i].votes == max_votes {
            println!("{}", candidates[i].name);
        }
    }
}