#[derive(Debug, Clone)]
struct Route {
    steps: Vec<Step>,
}

impl Route {
    fn new(steps: Vec<Step>) -> Route {
        Route { steps: steps }
    }

    fn close_portal(self: Self, n: usize) -> Route {
        let mut result = self.clone();
        result.steps[n] = match &result.steps[n] {
            Step::Portal(PortalState::Open, destination) => {
                Step::Portal(PortalState::Closed, *destination)
            }
            step => step.clone(),
        };
        result
    }
}

type StepIndex = usize;

#[derive(Debug, PartialEq, Clone)]
enum Step {
    Empty,
    Portal(PortalState, StepIndex),
}

#[derive(Debug, PartialEq, Clone)]
enum PortalState {
    Open,
    Closed,
}

#[derive(Debug, PartialEq)]
enum AntPosition {
    Start,
    Step(StepIndex),
    Finish,
}

#[derive(Debug)]
struct World {
    route: Route,
    ant_position: AntPosition,
}

impl World {
    fn new(steps: Vec<Step>) -> World {
        World {
            route: Route::new(steps),
            ant_position: AntPosition::Start,
        }
    }

    fn new_from_position(steps: Vec<Step>, ant_position: AntPosition) -> World {
        World {
            route: Route::new(steps),
            ant_position: ant_position,
        }
    }

    fn advance(self: &mut Self) {
        let mut route = self.route.clone();
        (self.ant_position, self.route) = match self.ant_position {
            AntPosition::Start => (AntPosition::Step(1), route),
            AntPosition::Step(n) => {
                if n == route.steps.len() {
                    (AntPosition::Finish, route)
                } else {
                    match route.steps[n] {
                        Step::Portal(PortalState::Open, destination) => {
                            (AntPosition::Step(destination), route.close_portal(n))
                        }
                        _ => (AntPosition::Step(n + 1), route),
                    }
                }
            }
            AntPosition::Finish => (AntPosition::Finish, route),
        }
    }
}

fn main() {
    let world: World = World::new(vec![
        Step::Empty,
        Step::Empty,
        Step::Empty,
        Step::Portal(PortalState::Closed, 3),
        Step::Empty,
        Step::Portal(PortalState::Open, 2),
        Step::Empty,
    ]);
    println!("{:?}", world);
}

#[test]
fn initial_ant_position_is_start() {
    let world: World = World::new(vec![
        Step::Empty,
        Step::Empty,
        Step::Empty,
        Step::Portal(PortalState::Closed, 3),
        Step::Empty,
        Step::Portal(PortalState::Open, 2),
        Step::Empty,
    ]);
    assert_eq!(world.ant_position, AntPosition::Start);
}

#[test]
fn ant_moves_one_step_at_a_time() {
    let mut world: World = World::new(vec![Step::Empty, Step::Empty, Step::Empty]);
    world.advance();
    assert_eq!(world.ant_position, AntPosition::Step(1));
    world.advance();
    assert_eq!(world.ant_position, AntPosition::Step(2));
    world.advance();
    assert_eq!(world.ant_position, AntPosition::Step(3));
}

#[test]
fn ant_moves_from_last_step_to_finish() {
    let mut world: World = World::new_from_position(
        vec![Step::Empty, Step::Empty, Step::Empty],
        AntPosition::Step(3),
    );
    world.advance();
    assert_eq!(world.ant_position, AntPosition::Finish);
}

#[test]
fn ant_does_not_move_past_finish() {
    let mut world: World = World::new_from_position(
        vec![Step::Empty, Step::Empty, Step::Empty],
        AntPosition::Finish,
    );
    world.advance();
    assert_eq!(world.ant_position, AntPosition::Finish);
}

#[test]
fn ant_walks_over_closed_portal() {
    let mut world: World = World::new_from_position(
        vec![
            Step::Empty,
            Step::Portal(PortalState::Closed, 1),
            Step::Empty,
        ],
        AntPosition::Step(1),
    );
    world.advance();
    assert_eq!(world.ant_position, AntPosition::Step(2));
    world.advance();
    assert_eq!(world.ant_position, AntPosition::Step(3));
}

#[test]
fn ant_is_teleported_by_open_portal() {
    let mut world: World = World::new_from_position(
        vec![Step::Empty, Step::Portal(PortalState::Open, 1), Step::Empty],
        AntPosition::Step(1),
    );
    world.advance();
    assert_eq!(world.ant_position, AntPosition::Step(1));
}

#[test]
fn portal_closes_when_teleporting() {
    let mut world: World = World::new_from_position(
        vec![Step::Empty, Step::Portal(PortalState::Open, 1), Step::Empty],
        AntPosition::Step(1),
    );
    world.advance();
    assert_eq!(world.route.steps[1], Step::Portal(PortalState::Closed, 1));
}
