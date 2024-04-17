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
		let mut mat = LedMatrix::new(m);
		mat.draw_matrix([[0;9];34]);
        mats.push(mat);
    }
    
    println!("Found LED matrix modules:");
    for i in mats.iter_mut() {
        println!("{} - {}", i.port_info.port_name.to_string(), i.get_fw_version());
    }

	let mut widgets: Vec<Box<dyn UpdatableWidget>> = Vec::with_capacity(3);
	widgets.push(Box::from(BatteryWidget::new()));
	widgets.push(Box::from(AllCPUsWidget::new(false)));
	widgets.push(Box::from(ClockWidget::new()));

    loop {
		let mut matrix = [[0;9];34];
		let mut offset_x = 0;
		let mut offset_y = 0;
		for widget in widgets.iter_mut() {
			widget.update();
			let size = widget.get_shape();
			matrix = matrix::emplace(matrix, widget, offset_x, offset_y);
			offset_y += size.y + 1;
		}
        mats[0].draw_matrix(matrix);
        thread::sleep(Duration::from_millis(2000));
    }

}

 
