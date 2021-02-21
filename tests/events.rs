extern crate nyx_space as nyx;

#[test]
fn event_tracker_true_anomaly() {
    use nyx::celestia::eclipse::EclipseLocator;
    use nyx::celestia::{Cosm, LTCorr, Orbit};
    use nyx::dynamics::orbital::OrbitalDynamics;
    use nyx::md::{Event, EventEvaluator, StateParameter};
    use nyx::propagators::error_ctrl::RSSStepPV;
    use nyx::propagators::*;
    use nyx::time::{Epoch, TimeUnit, J2000_OFFSET};
    use nyx::TimeTagged;

    let cosm = Cosm::de438();
    let eme2k = cosm.frame("EME2000");

    let dt = Epoch::from_mjd_tai(J2000_OFFSET);
    let state = Orbit::cartesian(
        -2436.45, -2436.45, 6891.037, 5.088_611, -5.088_611, 0.0, dt, eme2k,
    );

    let prop_time = state.period();

    // Track how many times we've passed by that TA again
    let peri_event = Event::new(StateParameter::Periapsis);
    let apo_event = Event::new(StateParameter::Apoapsis);
    let ta_event0 = Event::new(StateParameter::TrueAnomaly(35.1));
    let ta_event1 = Event::new(StateParameter::TrueAnomaly(235.1));

    let events = vec![peri_event, apo_event, ta_event0, ta_event1];

    let dynamics = OrbitalDynamics::two_body();
    let setup = Propagator::rk89(
        dynamics,
        PropOpts::with_adaptive_step_s(1.0, 60.0, 1e-9, RSSStepPV {}),
    );
    let mut prop = setup.with(state);
    let (_, traj) = prop.for_duration_with_traj(prop_time).unwrap();

    // Find all of the events
    for e in &events {
        let found_events = traj.find_all(e).unwrap();
        let pretty = found_events
            .iter()
            .map(|orbit| format!("{:o}\tevent value: {}\n", orbit, e.eval(orbit)))
            .collect::<String>();
        println!("[ta_tracker] {} =>\n{}", e, pretty);
    }

    // Find all eclipses?!
    let e_loc = EclipseLocator {
        light_source: cosm.frame("Sun J2000"),
        shadow_bodies: vec![cosm.frame("EME2000")],
        cosm,
        correction: LTCorr::None,
    };

    // Adding this print to confirm that the penumbra calculation continuously increases and then decreases.
    for state in traj.every(10 * TimeUnit::Second) {
        println!("{:o}\t{}", state, e_loc.compute(&state));
    }

    // let found_events = traj.find_all(&e_loc).unwrap();

    let found_events = traj.find_bracketed(
        Epoch::from_gregorian_tai(2002, 1, 1, 12, 28, 10, 0),
        Epoch::from_gregorian_tai(2002, 1, 1, 12, 30, 10, 0),
        &e_loc,
    );
    let pretty = found_events
        .iter()
        .map(|orbit| {
            format!(
                "{:o}\tevent value: {}\t(-10s: {}\t+10s: {})\n",
                orbit,
                &e_loc.compute(orbit),
                &e_loc.compute(
                    &traj
                        .evaluate(orbit.epoch() - 10 * TimeUnit::Second)
                        .unwrap()
                ),
                &e_loc.compute(
                    &traj
                        .evaluate(orbit.epoch() + 10 * TimeUnit::Second)
                        .unwrap()
                )
            )
        })
        .collect::<String>();
    println!("[eclipses] {} =>\n{}", e_loc, pretty);
}
