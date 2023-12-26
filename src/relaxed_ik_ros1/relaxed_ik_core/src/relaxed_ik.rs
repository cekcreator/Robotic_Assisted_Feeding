use crate::groove::vars::RelaxedIKVars;
use crate::groove::groove::{OptimizationEngineOpen};
use crate::groove::objective_master::ObjectiveMaster;
use crate::utils_rust::file_utils::{*};
use crate::utils_rust::transformations::{*};
use nalgebra::{Vector3, UnitQuaternion, Quaternion};
use std::os::raw::{c_double, c_int};

#[repr(C)]
pub struct Opt {
    pub data: *const c_double,
    pub length: c_int,
}

pub struct RelaxedIK {
    pub vars: RelaxedIKVars,
    pub om: ObjectiveMaster,
    pub groove: OptimizationEngineOpen,
    pub radius: f64,
    pub height: f64,
    pub obj_to_center_line: f64,
    pub start_cone: f64
}

impl RelaxedIK {
    pub fn load_settings( path_to_setting: &str) -> Self {
        println!("RelaxedIK is using below setting file {}", path_to_setting);

        let vars = RelaxedIKVars::from_local_settings(path_to_setting);
        // let om = ObjectiveMaster::relaxed_ik(&vars.robot.chain_lengths);
        let om = ObjectiveMaster::HIRO_ik(&vars.robot.chain_lengths);

        let groove = OptimizationEngineOpen::new(vars.robot.num_dofs.clone());

        let radius = 0.0;
        let height = 0.0;
        let obj_to_center_line = 0.0;
        let start_cone = 0.0;

        Self{vars, om, groove, radius, height, obj_to_center_line, start_cone}
    }

    pub fn reset(&mut self, x: Vec<f64>) {
        self.vars.reset( x.clone());
    }

    pub fn solve(&mut self) -> Vec<f64> {
        // println!("10 fuck u");
        let mut out_x = self.vars.xopt.clone();

        // println!("11 fuck u");
        self.groove.optimize(&mut out_x, &self.vars, &self.om, 100);
        // println!("12 fuck u");

        let frames = self.vars.robot.get_frames_immutable(&out_x);

        for i in 0..out_x.len() {
            if (out_x[i].is_nan()) {
                println!("No valid solution found! Returning previous solution: {:?}. End effector position goals: {:?}", self.vars.xopt, self.vars.goal_positions);
                return self.vars.xopt.clone();
            }
        }
        self.vars.update(out_x.clone());  
        out_x
    }
}
