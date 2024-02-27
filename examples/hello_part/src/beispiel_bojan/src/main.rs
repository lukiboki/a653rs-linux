// be aware! to use this on a µC, you probably need #![no_std] environment
use cas_calculation::utils::*;
use opencas::VAdvisory;
use uom::si::angle::degree;
use uom::si::f32::*;
use uom::si::length::foot;
use uom::si::velocity::{foot_per_minute, foot_per_second};

/*
fn main() {}
*/

pub fn calc_adv() -> VAdvisory { 
    // init AircraftState struct
    let mut ownship = AircraftState {
        groundspeed: Velocity::new::<foot_per_second>(500.0),
        vertical_speed: Velocity::new::<foot_per_minute>(150.0),
        lat: Angle::new::<degree>(52.1),
        lng: Angle::new::<degree>(10.5),
        altitude: Length::new::<foot>(5000.0),
        heading: Angle::new::<degree>(123.0),
    };

    let mut intruder = AircraftState {
        groundspeed: Velocity::new::<foot_per_second>(500.0),
        vertical_speed: Velocity::new::<foot_per_minute>(150.0),
        lat: Angle::new::<degree>(52.1),
        lng: Angle::new::<degree>(10.3),
        altitude: Length::new::<foot>(4500.0),
        heading: Angle::new::<degree>(100.0),
    };

    // init opencas instance (here vcas) - default state is COC
    let mut vcas = opencas::VCas {
        last_advisory: VAdvisory::ClearOfConflict,
    };

    // do your calculations needed for the network
    let rel_altitude = cas_calculation::utils::relative_altitudes(&ownship, &intruder);
    let tau_hori = cas_calculation::utils::calc_tau_horizontal(&ownship, &intruder);

    // Do CAS inference for VCAS
    let (vadvisory, _confidence) = vcas.process(
        rel_altitude,
        ownship.vertical_speed,
        intruder.vertical_speed,
        tau_hori,
    );
    println!("Current Advisory: {:#?}", vadvisory);
    vadvisory
}
