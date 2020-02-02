#[macro_use] extern crate graphplan;
use graphplan::Proposition;
use episodic::{StoryGenerator, Place, NarrativeArc};


fn main() {
    let init_state = hashset![
        Proposition::from("ship enabled"),
        Proposition::from("in orbit"),
        Proposition::from("at alien planet"),
    ];

    let place = Place::PlanetOrbit;
    let narrative_arc = NarrativeArc::Anomaly;

    println!("{:?}", StoryGenerator::make_story(&init_state, &place, &narrative_arc));
}
