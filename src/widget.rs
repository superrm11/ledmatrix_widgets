use chrono::{Local, Timelike};

const ON_FULL: u8 = 120;
const ON_DIM: u8 = 68;
const OFF: u8 = 0;

pub struct Shape {
    pub x: usize,
    pub y: usize,
}

/// A standard set of instructions for widgets that can be updated from the system
pub trait UpdatableWidget {
    fn update(&mut self);
    fn get_matrix(&mut self) -> Vec<u8>;
    fn get_shape(&mut self) -> Shape;
}

// ================ Frames ================
/// Battery frame with empty interior (9x4 shape)
const BAT_FRAME: &'static [u8] = [
    ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, ON_FULL, OFF, OFF, OFF,
    OFF, OFF, OFF, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, OFF, OFF, OFF, OFF, ON_FULL, ON_FULL, ON_FULL,
    ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF,
]
	.as_slice();

const DIGIT_0: &'static [u8] = [
	OFF, ON_FULL, OFF,
	ON_FULL, OFF, ON_FULL,
	ON_FULL, OFF, ON_FULL,
	ON_FULL, OFF, ON_FULL,
	OFF, ON_FULL, OFF
].as_slice();

const DIGIT_1: &'static [u8] = [
	OFF, OFF, ON_FULL,
	OFF, ON_DIM, ON_FULL,
	OFF, OFF, ON_FULL,
	OFF, OFF, ON_FULL,
	OFF, OFF, ON_FULL
].as_slice();

const DIGIT_2: &'static [u8] = [
	ON_FULL, ON_FULL, ON_FULL,
	OFF, OFF, ON_FULL,
	ON_FULL, ON_FULL, ON_FULL,
	ON_FULL, OFF, OFF,
	ON_FULL, ON_FULL, ON_FULL
].as_slice();

const DIGIT_3: &'static [u8] = [
	ON_FULL, ON_FULL, ON_FULL,
	OFF, OFF, ON_FULL,
	ON_FULL, ON_FULL, OFF,
	OFF, OFF, ON_FULL,
	ON_FULL, ON_FULL, ON_FULL
].as_slice();

const DIGIT_4: &'static [u8] = [
	ON_FULL, OFF, ON_FULL,
	ON_FULL, OFF, ON_FULL,
	ON_FULL, ON_FULL, ON_FULL,
	OFF, OFF, ON_FULL,
	OFF, OFF, ON_FULL
].as_slice();

const DIGIT_5: &'static [u8] = [
	ON_FULL, ON_FULL, ON_FULL,
	ON_FULL, OFF, OFF,
	ON_FULL, ON_FULL, ON_FULL,
	OFF, OFF, ON_FULL,
	ON_FULL, ON_FULL, ON_FULL
].as_slice();

const DIGIT_6: &'static [u8] = [
	OFF, ON_FULL, ON_DIM,
	ON_FULL, OFF, OFF,
	ON_FULL, ON_FULL, ON_FULL,
	ON_FULL, OFF, ON_FULL,
	ON_FULL, ON_FULL, ON_FULL
].as_slice();

const DIGIT_7: &'static [u8] = [
	ON_FULL, ON_FULL, ON_FULL,
	ON_DIM, OFF, ON_FULL,
	OFF, OFF, ON_FULL,
	OFF, ON_FULL, OFF,
	OFF, ON_FULL, OFF
].as_slice();

const DIGIT_8: &'static [u8] = [
	ON_FULL, ON_FULL, ON_FULL,
	ON_FULL, OFF, ON_FULL,
	ON_FULL, ON_FULL, ON_FULL,
	ON_FULL, OFF, ON_FULL,
	ON_FULL, ON_FULL, ON_FULL
].as_slice();

const DIGIT_9: &'static [u8] = [
	ON_FULL, ON_FULL, ON_FULL,
	ON_FULL, OFF, ON_FULL,
	ON_FULL, ON_FULL, ON_FULL,
	OFF, OFF, ON_FULL,
	ON_DIM, ON_FULL, OFF
].as_slice();

// ================ Widgets ================
/// -------- Battery Widget --------
/// Create a widget that displays the battery remaining in the laptop
pub struct BatteryWidget {
    bat_level_pct: f32
}

impl BatteryWidget {
    pub fn new() -> BatteryWidget {
        println!("Initializing BatteryWidget");
        BatteryWidget {
            bat_level_pct: 0.0,
        }
    }
}

impl UpdatableWidget for BatteryWidget {
    fn update(&mut self) {
        // Update the battery percentage
        self.bat_level_pct = battery::Manager::new()
            .unwrap()
            .batteries()
            .unwrap()
            .enumerate()
            .next()
            .unwrap()
            .1
            .unwrap()
            .state_of_charge()
            .get::<battery::units::ratio::percent>();
    }

    fn get_matrix(&mut self) -> Vec<u8> {
        
        // Create the matrix
        let mut out: Vec<u8> = Vec::new();
        out.extend_from_slice(BAT_FRAME);

        let num_illum = (self.bat_level_pct * 6.0 / 100.0).round();

        for i in 1..7 {
            if i <= num_illum as usize {
                out[(self.get_shape().x) + i]= ON_DIM;
                out[(self.get_shape().x * 2) + i]= ON_DIM;
            }
        }
        
        out
    }

    fn get_shape(&mut self) -> Shape {
        return Shape { x: 9, y: 4 };
    }
}

// -------- All Cores CPU Usage Widget --------
/// Create a widget that displays the usage of all CPU cores, one per row.
pub struct AllCPUsWidget {
    cpu_usages: Vec<u8>,
    sys: sysinfo::System
}

impl AllCPUsWidget {
    pub fn new() -> AllCPUsWidget {
        let mut newsys = sysinfo::System::new();
        newsys.refresh_cpu();

        println!("Initializing AllCPUsWidget");

        AllCPUsWidget {
            cpu_usages: vec![0; newsys.cpus().len()],
            sys: newsys
        }
    }
}

impl UpdatableWidget for AllCPUsWidget {

    fn update(&mut self) {
        // Refresh the cpu usage
        self.sys.refresh_cpu();

        for i in 0..self.sys.cpus().len() {
            self.cpu_usages[i] = self.sys.cpus()[i].cpu_usage().round() as u8;
        }
    }

    /// Refresh the CPU usage and redraw the matrix
    fn get_matrix(&mut self) -> Vec<u8> {

        // Create the matrix
        let width = self.get_shape().x;
        let mut out = vec![0;self.cpu_usages.len() * width];

        for y in 0..self.cpu_usages.len() {
            for x in 0..self.get_shape().x {
                if x <= (self.cpu_usages[y] as f32 * width as f32 / 100.0) as usize {
                    out[x + (y * width)] = ON_FULL;
                }
            }         
        }

        out
    }

    fn get_shape(&mut self) -> Shape {
        return Shape {x: 9, y:self.cpu_usages.len()};
    }
}

pub struct ClockWidget {
	time: chrono::DateTime<Local>
}

impl ClockWidget {
	pub fn new() -> Self {
		println!("Initializing ClockWidget");
		let dt = chrono::offset::Local::now();
		Self {
			time: dt,
		}
	}

	fn render_digit(num: u32) -> &'static [u8] {
		 match num {
			0 => DIGIT_0,
			1 => DIGIT_1,
			2 => DIGIT_2,
			3 => DIGIT_3,
			4 => DIGIT_4,
			5 => DIGIT_5,
			6 => DIGIT_6,
			7 => DIGIT_7,
			8 => DIGIT_8,
			9 => DIGIT_9,
			_ => DIGIT_0,
		}
	}

	fn render_number(num: u32) -> Vec<u8> {
		let mut numrow = vec![0; 9 * 5];
		let first_digit = Self::render_digit(num / 10);
		let second_digit = Self::render_digit(num % 10);
		for idx in 0..(9*5) {
			let cell = match idx % 9 {
				1 | 2 | 3 => {
					first_digit[((idx / 9) * 3) + (idx % 9) - 1]
				},
				5 | 6 | 7 => {
					second_digit[((idx / 9) * 3) + idx % 9 - 5]
				},
				_ => { OFF }
			};
			numrow[idx] = cell;
		};
		numrow
	}
}

impl UpdatableWidget for ClockWidget {
    fn update(&mut self) {
        self.time = chrono::offset::Local::now();
    }

    fn get_matrix(&mut self) -> Vec<u8> {
		let mut matrix = Vec::with_capacity(9 * 11);
		matrix.extend(Self::render_number(self.time.hour()));
		matrix.extend(vec![OFF; 9]);
		matrix.extend(Self::render_number(self.time.minute()));
		matrix
    }

    fn get_shape(&mut self) -> Shape {
        return Shape{x: 9, y: 11}
    }
}
