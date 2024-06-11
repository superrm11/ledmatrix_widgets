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

use crate::widget::{AllCPUsWidget, BatteryWidget, ClockWidget, UpdatableWidget};

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    // ======== Info about system ========
    #[arg(long)]
    /// List all connected matrix modules
    list_modules: bool,

    /// List all widgets available for placement
    #[arg(long)]
    list_widgets: bool, // ======== Program Control ========
                        // #[arg(long)]
                        // Start the background service updating the matrix
                        // start: bool,

                        // #[arg(long)]
                        // JSON config file path
                        // config: Option<String>,
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
            if mats.len() == 0 {
                println!("No modules found, unable to continue.");
                exit(1);
            }

            // No arguments provided? Start the
            if args().len() <= 1 {
                let mut bat = BatteryWidget::new();
                let mut cpu = AllCPUsWidget::new(false);
                let mut clock = ClockWidget::new();

                let blank = [[0; 9]; 34];

                if mats.len() == 2 {
                    mats[1].draw_matrix(blank);
                }

                loop {
                    bat.update();
                    cpu.update();
                    clock.update();

                    let mut matrix = [[0; 9]; 34];
                    matrix = matrix::emplace(matrix, &bat, 0, 0);
                    matrix = matrix::emplace(matrix, &cpu, 0, 5);
                    matrix = matrix::emplace(matrix, &clock, 0, 23);
                    mats[0].draw_matrix(matrix);
                    thread::sleep(Duration::from_millis(2000));
                }
            }
        }
        Program::ListMod => {
            LedMatrix::detect();
        }
        Program::ListWid => {
            println!(
                "Battery Indicator:\n \
                A 9x4 widget in the shape of a battery, with an internal bar indicating remaining capacity.\n"
            );
            println!(
                "CPU Usage Indicator:\n \
                A 9x16 widget where each row of LEDs is a bar that represents the CPU usage of one core.\n"
            );
            println!(
                "Clock Widget:\n \
                A 9x11 widget that displays the system time in 24hr format.\n"
            );
        } // _ => {}
    }

    exit(0);
}
