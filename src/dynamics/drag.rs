use super::na::Vector3;
use super::ForceModel;
use celestia::{bodies, Cosm, Geoid, State};

/// `ConstantDrag` implements a constant drag model as defined in Vallado, 4th ed., page 551, with an important caveat.
///
/// **WARNING:** This basic model assumes that the velocity of the spacecraft is identical to the velocity of the upper atmosphere,
/// This is a **bad** assumption and **should not** be used for high fidelity simulations.
/// This will be resolved after https://gitlab.com/chrisrabotin/nyx/issues/93 is implemented.
#[derive(Clone)]
pub struct ConstantDrag<'a> {
    /// in m^2
    pub sc_area: f64,
    /// coefficient of drag; (spheres are between 2.0 and 2.1, use 2.2 in Earth's atmosphere).
    pub cd: f64,
    /// atmospheric density in kg/m^3
    pub rho: f64,
    /// Geoid causing the drag
    pub drag_geoid: Geoid,
    /// a Cosm reference is needed to convert to the state around the correct planet
    pub cosm: &'a Cosm,
}

impl<'a> ForceModel<Geoid> for ConstantDrag<'a> {
    fn eom(&self, osc: &State<Geoid>) -> Vector3<f64> {
        let osc = self.cosm.frame_chg(&osc, self.drag_geoid);
        let velocity = osc.velocity();
        -0.5 * self.rho * self.cd * self.sc_area * velocity.norm() * velocity
    }
}

/// `ExpEarthDrag` implements an exponential decay drag model.
///
/// **WARNING:** This model assumes that the velocity of the spacecraft is identical to the velocity of the upper atmosphere,
/// This is a **bad** assumption and **should not** be used for high fidelity simulations.
/// /// This will be resolved after https://gitlab.com/chrisrabotin/nyx/issues/93 is implemented.
#[derive(Clone)]
pub struct ExpEarthDrag<'a> {
    /// in m^2
    pub sc_area: f64,
    /// coefficient of drag; (spheres are between 2.0 and 2.1, use 2.2 in Earth's atmosphere).
    pub cd: f64,
    /// a Cosm reference is needed to convert to the state around the correct planet
    pub cosm: &'a Cosm,
}

impl<'a> ForceModel<Geoid> for ExpEarthDrag<'a> {
    fn eom(&self, osc: &State<Geoid>) -> Vector3<f64> {
        let earth = self.cosm.geoid_from_id(bodies::EARTH);
        // Compute the density
        let rho0 = 3.614e-13; // # kg/m^3
        let r0 = 700_000.0 + earth.equatorial_radius;
        let h = 88_667.0; // m
        let rho = rho0 * (-(osc.rmag() - r0) / h).exp(); // # Exponential decay model for density

        let osc = self.cosm.frame_chg(&osc, earth);
        let velocity = osc.velocity();
        -0.5 * rho * self.cd * self.sc_area * velocity.norm() * velocity
    }
}
