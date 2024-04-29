mod ledmatrix;
mod matrix;
mod widget;
use std::{
    env::{args, args_os},
    process::exit,
    thread,
    time::Duration,
};

use clap::Parser;
use ledmatrix::LedMatrix;

use crate::widget::{AllCPUsWidget, BatteryWidget, UpdatableWidget};

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    // ======== Info about system ========
    #[arg(long)]
    /// List all connected matrix modules
    list_modules: bool,

    /// List all widgets available for placement
    #[arg(long)]
    list_widgets: bool,

    // ======== Program Control ========
    #[arg(long)]
    /// Start the background service updating the matrix
    start: bool,

    #[arg(long)]
    /// JSON config file path
    config: Option<String>,
}

enum Program {
    ListMod,
    ListWid,
    Default,
}

fn main() {
    // TODO possible options:
    // each widget + Y placement + LED module (both as default) (for now, x maybe later)
    // Overall brightness
    // update rate

    let mut program = Program::Default;

    if args_os().len() > 1 {
        let cli = Cli::parse();
        if cli.list_modules {
            program = Program::ListMod;
        } else if cli.list_widgets {
            program = Program::ListWid;
        }
    }

    match program {
        Program::Default => {
            let mut mats = LedMatrix::detect();

            // No arguments provided? Start the
            if args().len() <= 1 {
                let mut b = BatteryWidget::new();
                let mut c = AllCPUsWidget::new();

                let blank = [[0; 9]; 34];
                mats[1].draw_matrix(blank);

                loop {
                    b.update();
                    c.update();

                    let mut matrix = [[0; 9]; 34];
                    matrix = matrix::emplace(matrix, Box::from(&mut b), 0, 0);
                    matrix = matrix::emplace(matrix, Box::from(&mut c), 0, 5);
                    mats[0].draw_matrix(matrix);
                    thread::sleep(Duration::from_millis(2000));
                }
            }
        },
        Program::ListMod => {
            LedMatrix::detect();
        },
        Program::ListWid => println!("Not yet implemented :')"),

        // _ => {}
    }

    exit(0);
}
