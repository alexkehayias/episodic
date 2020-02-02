// TODO: Remove this
#![allow(dead_code, unused_variables)]

use std::collections::{HashSet, VecDeque};

use rand;
use rand::Rng;
#[macro_use] extern crate graphplan;
use graphplan::Proposition;
#[macro_use] extern crate lazy_static;


#[derive(Debug, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub enum Action {
    Scan,
    EngineMalfunction,
    ComputerMalfunction,
    PowerMalfunction,
    DamageHull,
    ScienceBreakthrough,
    WeaponsBreakthrough,
    RemediateAnomaly,
    Repair,
}

lazy_static! {
    pub static ref ALL_ACTIONS: HashSet<graphplan::Action<Action, &'static str>> = {
        let mut actions = HashSet::new();

        let p1 = Proposition::from("ship enabled");
        let p2 = Proposition::from("anomaly detected");
        actions.insert(
            graphplan::Action::new(
                Action::Scan,
                hashset!{&p1},
                hashset!{&p2}
            )
        );

        let p3 = p1.negate();
        actions.insert(
            graphplan::Action::new(
                Action::EngineMalfunction,
                hashset!{&p1},
                hashset!{&p3}
            )
        );
        actions.insert(
            graphplan::Action::new(
                Action::ComputerMalfunction,
                hashset!{&p1},
                hashset!{&p3}
            )
        );
        actions.insert(
            graphplan::Action::new(
                Action::PowerMalfunction,
                hashset!{&p1},
                hashset!{&p3}
            )
        );

        let p4 = Proposition::from("hull breach imminent");
        actions.insert(
            graphplan::Action::new(
                Action::DamageHull,
                hashset!{},
                hashset!{&p4}
            )
        );

        let p6 = Proposition::from("anomaly plan");
        actions.insert(
            graphplan::Action::new(
                Action::ScienceBreakthrough,
                hashset!{&p4, &p4},
                hashset!{&p6}
            )
        );
        actions.insert(
            graphplan::Action::new(
                Action::WeaponsBreakthrough,
                hashset!{&p4, &p4},
                hashset!{&p6}
            )
        );

        let p7 = Proposition::from("anomaly destroyed");
        actions.insert(
            graphplan::Action::new(
                Action::RemediateAnomaly,
                hashset!{&p6},
                hashset!{&p7}
            )
        );

        actions.insert(
            graphplan::Action::new(
                Action::Repair,
                hashset!{&p3},
                hashset!{&p1}
            )
        );

        actions
    };
}

#[derive(Debug)]
pub enum NarrativeArc {
    Anomaly,
    Chase,
    Combat,
    Crime,
    Defense,
    Diplomacy,
    Discovery,
    DistressCall,
    FirstContact,
    Illness,
    MissingCrewMember,
    Radiation,
}

const ALL_NARRATIVE_ARC_ELEMENTS_COUNT: usize = 12;
static ALL_NARRATIVE_ARC_ELEMENTS: [NarrativeArc; ALL_NARRATIVE_ARC_ELEMENTS_COUNT] = [
    NarrativeArc::Anomaly,
    NarrativeArc::Chase,
    NarrativeArc::Combat,
    NarrativeArc::Crime,
    NarrativeArc::Defense,
    NarrativeArc::Diplomacy,
    NarrativeArc::Discovery,
    NarrativeArc::DistressCall,
    NarrativeArc::FirstContact,
    NarrativeArc::Illness,
    NarrativeArc::MissingCrewMember,
    NarrativeArc::Radiation,
];

#[derive(Debug)]
pub enum Place {
    Bridge,
    Engineering,
    PlanetOrbit,
    PlanetSurface,
    ResearchStation,
    ScienceOutpost,
    SickBay,
    SpaceStation,
}

const ALL_PLACES_COUNT: usize = 8;
static ALL_PLACES: [Place; ALL_PLACES_COUNT] = [
    Place::Bridge,
    Place::Engineering,
    Place::PlanetOrbit,
    Place::PlanetSurface,
    Place::ResearchStation,
    Place::ScienceOutpost,
    Place::SickBay,
    Place::SpaceStation,
];

#[derive(Debug)]
pub struct Story<'s> {
    narrative_arc: &'s NarrativeArc,
    place: &'s Place,
    plot_points: Vec<Action>,
}

impl<'s> Story<'s> {
    pub fn new(narrative_arc: &'s NarrativeArc, place: &'s Place, plot_points: Vec<Action>) -> Self {
        Story {
            narrative_arc,
            place,
            plot_points,
        }
    }
}

/// Generates a `Story`
#[derive(Debug)]
pub struct StoryGenerator {
    rng: rand::rngs::ThreadRng,
}

impl StoryGenerator {
    pub fn new() -> Self {
        let rng = rand::thread_rng();
        Self { rng: rng }
    }

    /// Generates a sequence of plot points that should represent a
    /// plot-centric, coherent narrative
    pub fn plot_points(
        init_state: &HashSet<Proposition<&str>>,
        narrative_arc: &NarrativeArc,
        place: &Place,
    ) -> Vec<Action> {
        let (setup, incident, escalation, climax, resolution, end) = match narrative_arc {
            // TODO introduce variation within the narrative_arc
            NarrativeArc::Anomaly => (
                hashset![Proposition::from("anomaly detected")],
                hashset![Proposition::from("ship enabled").negate()],
                hashset![Proposition::from("hull breach imminent")],
                hashset![Proposition::from("anomaly plan")],
                hashset![Proposition::from("anomaly destroyed")],
                hashset![Proposition::from("ship enabled")]),
            _ => todo!("NarrativeArc not implemented yet")
        };
        // TODO: Maybe some reward or consequence? Player fails to
        // subvert the anomaly, mediator steps in to resolve and
        // applies a consequence e.g. crew member lost, damaged system
        // Reward could be something like extra experience points, new
        // discovery that improves some aspect of the ship or crew
        // member

        // Derive the plot points from the state changes. Build a
        // sequence of actions from each stage (init, incident,
        // escalation, etc.).
        let init: HashSet<Proposition<_>> = setup.union(init_state).cloned().collect();
        let mut plot_points: Vec<Action> = Vec::new();
        let mut state = init_state.to_owned();

        for goals in [setup, incident, escalation, climax, resolution, end].iter() {
            let mut graphplan = graphplan::GraphPlan::new(
                state.clone(),
                goals.clone(),
                ALL_ACTIONS.clone(),
                graphplan::SimpleSolver,
            );

            // TODO: Yield multiple plans and choose one randomly
            let (plan, end_state) = graphplan.search().and_then(|plan| {
                // Graphplan returns a vector of
                // hashsets. Since we don't care about
                // parallel actions we can linearize and still
                // be correct
                let mut translated_plan = VecDeque::new();
                let mut end_state: HashSet<Proposition<_>> = HashSet::new();
                for action in plan.iter().flat_map(|step| step.iter()) {
                    translated_plan.push_back(action.get_action().to_owned());
                    for prop in &action.effects {
                        end_state.insert(prop.clone());
                    }

                }

                Some((translated_plan, end_state))
            }).expect(&format!("No plan found for init: {:?} end: {:?}",
                               state, goals));

            // Accumulate the results
            dbg!(&state, &goals, &plan);
            for action in plan {
                plot_points.push(action)
            }

            // If any of the end states from a successful plan are a
            // negation of the initial state, override with the end
            // state. This allows us to properly accumulate state
            // between plans.
            for e in &end_state {
                let negation = e.negate();
                if state.contains(&negation) {
                    state.remove(&negation);
                }
            }
            state = state.union(&end_state).cloned().collect();
        }

        plot_points
    }

    pub fn make_story<'s>(init_state: &'s HashSet<Proposition<&str>>,
                          place: &'s Place,
                          narrative_arc: &'s NarrativeArc) -> Story<'s> {
        // Generate the essential plot points
        let plot_points = Self::plot_points(init_state, narrative_arc, place);

        // TODO: maybe generate an initial graph based on the end state?

        Story::new(narrative_arc, place, plot_points)
    }

    pub fn make_random_story<'s>(&mut self, init_state: &'s HashSet<Proposition<&str>>) -> Story<'s> {
        // Randomly select elements of the story
        let place_idx = self.rng.gen_range(0, ALL_PLACES_COUNT);
        let place = &ALL_PLACES[place_idx];
        let narrative_arc_idx = self.rng.gen_range(0, ALL_NARRATIVE_ARC_ELEMENTS_COUNT);
        let narrative_arc = &ALL_NARRATIVE_ARC_ELEMENTS[narrative_arc_idx];

        Self::make_story(init_state, place, narrative_arc)
    }
}

/// Interprets the inputs from agents and updates the Narrative
struct StoryMediator {}

impl StoryMediator {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_story_works() {
        let init_state = hashset![
            Proposition::from("ship enabled"),
            Proposition::from("in orbit"),
            Proposition::from("at alien planet"),
        ];

        let place = Place::PlanetOrbit;
        let narrative_arc = NarrativeArc::Anomaly;

        StoryGenerator::make_story(&init_state, &place, &narrative_arc);
    }
}
