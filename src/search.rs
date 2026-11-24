use crate::track::Track;
use nucleo_matcher::{
    Config, Matcher,
    pattern::{CaseMatching, Normalization, Pattern},
};

fn search(input: String, tracks: Vec<Track>) -> Vec<(Track, u32)> {
    //let input: String = input.to_lowercase();
    //or track in &tracks {
    //    println!("{:?}", track.haystack);
    //}

    //create a new matcher with default config, see the nucleo docs for default config.
    //next, define matches, which is a Vec of the Track and the score the Track got. requires
    //AsRef<str>, implemented in my Track object and simply returns the Track's haystack as a str.
    let mut matcher: Matcher = Matcher::new(Config::DEFAULT.match_paths());

    //scored matches
    Pattern::parse(&input, CaseMatching::Ignore, Normalization::Smart)
        .match_list(tracks, &mut matcher)
}
