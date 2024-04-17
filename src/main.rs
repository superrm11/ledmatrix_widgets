mod ledmatrix;
mod matrix;
mod widget;
use std::{process::exit, thread, time::Duration};

use ledmatrix::LedMatrix;

use crate::widget::{AllCPUsWidget, BatteryWidget, ClockWidget, UpdatableWidget};

fn main() {
    // TODO possible options: 
    // each widget + Y placement + LED module (both as default) (for now, x maybe later)
    // Overall brightness
    // update rate

    let mats_info = LedMatrix::detect();

    if mats_info.len() <= 0 {
        println!("No LED matrix modules found.");
        exit(1)
    }

    let mut mats: Vec<LedMatrix> = Vec::new();
    for m in mats_info {
        mats.push(LedMatrix::new(m));
    }
    
    println!("Found LED matrix modules:");
    for i in mats.iter_mut() {
        println!("{} - {}", i.port_info.port_name.to_string(), i.get_fw_version());
    }

    let mut b = BatteryWidget::new();
    let mut c = AllCPUsWidget::new();
	let mut clock = ClockWidget::new();

    let blank = [[0;9];34];
    mats[0].draw_matrix(blank);

    loop {
        b.update();
        c.update();

        let mut matrix = [[0;9];34];
        matrix = matrix::emplace(matrix, Box::from(&mut b), 0, 0);
        matrix = matrix::emplace(matrix, Box::from(&mut c), 0, 5);
		matrix = matrix::emplace(matrix, Box::from(&mut clock), 0, 22);
        mats[0].draw_matrix(matrix);
        thread::sleep(Duration::from_millis(2000));
    }

}

 
