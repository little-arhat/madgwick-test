extern crate csv;
extern crate madgwick;

use madgwick::{Marg, Vector3};
// use madgwick::{F32x3, Marg};

use std::io;
use std::str::FromStr;

const SAMPLE_FREQ: f32 = 512.0; // sample frequency in Hz
const BETA_DEF: f32 = 0.1; // 2 * proportional gain

fn parse_float(s: &str) -> f32 {
    f32::from_str(s).unwrap()
}

fn parse_accels(sr: &csv::StringRecord) -> Vector3<f32> {
    Vector3::new(
        parse_float(sr.get(0).unwrap()),
        parse_float(sr.get(1).unwrap()),
        parse_float(sr.get(2).unwrap()),
    )
}

// fn parse_accels(sr: &csv::StringRecord) -> F32x3 {
//     F32x3 {
//         x: parse_float(sr.get(0).unwrap()),
//         y: parse_float(sr.get(1).unwrap()),
//         z: parse_float(sr.get(2).unwrap()),
//     }
// }

fn main() {
    let g = Vector3::new(0., 0., 0.);
    // let g = F32x3 {
    //     x: 0.,
    //     y: 0.,
    //     z: 0.,
    // };
    let mut m = Marg::new(BETA_DEF, SAMPLE_FREQ);
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result.unwrap();
        let a = parse_accels(&record);
        let q = m.update_imu(g, a);
        println!("{},{},{},{}", q.w, q.i, q.j, q.k);
        // println!("{},{},{},{}", q.0, q.1, q.2, q.3)
    }
}
