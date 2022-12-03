use super::*;

#[test]
fn rock_beats_scissors() {
    assert_eq!(score(ROCK, SCISSORS), 6);
    assert_eq!(score(SCISSORS, ROCK), 0);
}

#[test]
fn scissors_beats_paper() {
    assert_eq!(score(SCISSORS, PAPER), 6);
    assert_eq!(score(PAPER, SCISSORS), 0);
}

#[test]
fn paper_beats_rock() {
    assert_eq!(score(ROCK, PAPER), 0);
    assert_eq!(score(PAPER, ROCK), 6);
}

#[test]
fn everything_draws_against_itself() {
    assert_eq!(score(ROCK, ROCK), 3);
    assert_eq!(score(PAPER, PAPER), 3);
    assert_eq!(score(SCISSORS, SCISSORS), 3);
}