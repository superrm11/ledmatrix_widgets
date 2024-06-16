use chrono::{Local, Timelike};

const ON_FULL: u8 = 120;
const ON_DIM: u8 = 68;
const OFF: u8 = 0;

#[derive(Clone)]
pub struct Shape {
    pub x: usize,
    pub y: usize,
}

/// A standard set of instructions for widgets that can be updated from the system
pub trait UpdatableWidget {
    fn update(&mut self);
    fn get_matrix(&self) -> &Vec<u8>;
    fn get_shape(&self) -> &Shape;
}

// ================ Frames ================
/// Battery frame with empty interior (9x4 shape)
const BAT_FRAME: &'static [u8] = [
    ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, ON_FULL, OFF, OFF,
    OFF, OFF, OFF, OFF, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, OFF, OFF, OFF, OFF, ON_FULL, ON_FULL,
    ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF,
]
.as_slice();

const DIGIT_0: &'static [u8] = [
    OFF, ON_FULL, OFF, ON_FULL, OFF, ON_FULL, ON_FULL, OFF, ON_FULL, ON_FULL, OFF, ON_FULL, OFF,
    ON_FULL, OFF,
]
.as_slice();

const DIGIT_1: &'static [u8] = [
    OFF, OFF, ON_FULL, OFF, ON_DIM, ON_FULL, OFF, OFF, ON_FULL, OFF, OFF, ON_FULL, OFF, OFF,
    ON_FULL,
]
.as_slice();

const DIGIT_2: &'static [u8] = [
    ON_FULL, ON_FULL, ON_FULL, OFF, OFF, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, OFF,
    ON_FULL, ON_FULL, ON_FULL,
]
.as_slice();

const DIGIT_3: &'static [u8] = [
    ON_FULL, ON_FULL, ON_FULL, OFF, OFF, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, OFF, ON_FULL,
    ON_FULL, ON_FULL, ON_FULL,
]
.as_slice();

const DIGIT_4: &'static [u8] = [
    ON_FULL, OFF, ON_FULL, ON_FULL, OFF, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, ON_FULL,
    OFF, OFF, ON_FULL,
]
.as_slice();

const DIGIT_5: &'static [u8] = [
    ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, ON_FULL,
    ON_FULL, ON_FULL, ON_FULL,
]
.as_slice();

const DIGIT_6: &'static [u8] = [
    OFF, ON_FULL, ON_DIM, ON_FULL, OFF, OFF, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, ON_FULL,
    ON_FULL, ON_FULL, ON_FULL,
]
.as_slice();

const DIGIT_7: &'static [u8] = [
    ON_FULL, ON_FULL, ON_FULL, ON_DIM, OFF, ON_FULL, OFF, OFF, ON_FULL, OFF, ON_FULL, OFF, OFF,
    ON_FULL, OFF,
]
.as_slice();

const DIGIT_8: &'static [u8] = [
    ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, ON_FULL, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF,
    ON_FULL, ON_FULL, ON_FULL, ON_FULL,
]
.as_slice();

const DIGIT_9: &'static [u8] = [
    ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, ON_FULL, ON_FULL, ON_FULL, ON_FULL, OFF, OFF, ON_FULL,
    ON_DIM, ON_FULL, OFF,
]
.as_slice();

// ================ Widgets ================
/// -------- Battery Widget --------
/// Create a widget that displays the battery remaining in the laptop
pub struct BatteryWidget {
    matrix: Vec<u8>,
    shape: Shape,
    chrg_ind: bool
}

impl BatteryWidget {
    pub fn new() -> BatteryWidget {
        println!("Initializing BatteryWidget");
        BatteryWidget { 
            matrix: vec![], 
            chrg_ind: false,
            shape: Shape{x: 9, y:4}
        }
    }
}

impl UpdatableWidget for BatteryWidget {
    fn update(&mut self) {
        // Update the battery percentage
        let battery_dev = battery::Manager::new()
            .unwrap()
            .batteries()
            .unwrap()
            .enumerate()
            .next()
            .unwrap()
            .1
            .unwrap();

        // Update whether or not the device is charging
        let bat_level_pct = battery_dev.state_of_charge()
            .get::<battery::units::ratio::percent>();
        
        let is_charging = battery_dev.state() == battery::State::Charging;

        // Recreate the matrix
        self.matrix = vec![];
        self.matrix.extend_from_slice(BAT_FRAME);

        let num_illum = (bat_level_pct * 6.0 / 100.0).round();

        // Fill battery bar
        for i in 1..7 {
            if i <= num_illum as usize {
                self.matrix[(self.shape.x) + i] = ON_DIM;
                self.matrix[(self.shape.x * 2) + i] = ON_DIM;
            }
        }

        // Charging indicator
        if is_charging && bat_level_pct < 99.0 {
            self.matrix[self.shape.x + num_illum as usize] = if self.chrg_ind {ON_DIM} else {OFF};
            self.matrix[(2*self.shape.x) + num_illum as usize] = if self.chrg_ind {ON_DIM} else {OFF};
            self.chrg_ind = !self.chrg_ind;
        }
    }

    fn get_matrix(&self) -> &Vec<u8> {
        &self.matrix
    }

    fn get_shape(&self) -> &Shape {
        &self.shape
    }
}

// -------- All Cores CPU Usage Widget --------
/// Create a widget that displays the usage of all CPU cores, one per row.
pub struct AllCPUsWidget {
    cpu_usages: Vec<u8>,
    merge_threads: bool,
    sys: sysinfo::System,
    matrix: Vec<u8>,
    shape: Shape
}

impl AllCPUsWidget {
    pub fn new(merge_threads: bool) -> AllCPUsWidget {
        let mut newsys = sysinfo::System::new();
        newsys.refresh_cpu();

        println!("Initializing AllCPUsWidget");

        AllCPUsWidget {
            shape: match merge_threads {
                false => Shape {
                    x: 9,
                    y: newsys.cpus().len(),
                },
                true => Shape { x: 8, y: 8 },
            },
            cpu_usages: vec![0; newsys.cpus().len()],
            merge_threads,
            sys: newsys,
            matrix: vec![],
        }
    }
}

impl UpdatableWidget for AllCPUsWidget {
    fn update(&mut self) {
        // Refresh the cpu usage
        self.sys.refresh_cpu();

        for (idx, usage) in self.sys.cpus().iter().enumerate() {
            self.cpu_usages[idx] = usage.cpu_usage().round() as u8;
        }

        // Create the matrix
        let width = self.get_shape().x;
		let height = self.get_shape().y;
        self.matrix = vec![OFF; width * height];

        if self.merge_threads {
            for idy in 0..height {
				let inverse_y = height - (idy + 1);
                for (idx, chunk) in self.cpu_usages.chunks(2).enumerate() {
                    let usage = (chunk[0] + chunk[1]) / 2;
					if usage as usize >= inverse_y * 10 {
						self.matrix[(idy * width) + idx] = ON_FULL;
					}
                }
            }
        } else {
            for y in 0..16 {
                for x in 0..width {
                    if x <= (self.cpu_usages[y] as f32 * width as f32 / 100f32) as usize {
                        self.matrix[x + (y * width)] = ON_FULL;
                    }
                }
            }
        }
    }

    fn get_matrix(&self) -> &Vec<u8> {
        &self.matrix
    }

    fn get_shape(&self) -> &Shape {
        &self.shape
    }
}

pub struct ClockWidget {
    matrix: Vec<u8>,
    time: chrono::DateTime<Local>,
}

impl ClockWidget {
    pub fn new() -> Self {
        println!("Initializing ClockWidget");
        let dt = chrono::offset::Local::now();
        Self { time: dt, matrix: vec![] }
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
        for idx in 0..(9 * 5) {
            let cell = match idx % 9 {
                1 | 2 | 3 => first_digit[((idx / 9) * 3) + (idx % 9) - 1],
                5 | 6 | 7 => second_digit[((idx / 9) * 3) + idx % 9 - 5],
                _ => OFF,
            };
            numrow[idx] = cell;
        }
        numrow
    }
}

impl UpdatableWidget for ClockWidget {
    fn update(&mut self) {
        self.time = chrono::offset::Local::now();
        self.matrix = Vec::with_capacity(9 * 11);
        self.matrix.extend(Self::render_number(self.time.hour()));
        self.matrix.extend(vec![OFF; 9]);
        self.matrix.extend(Self::render_number(self.time.minute()));
        
    }

    fn get_matrix(&self) -> &Vec<u8> {
        &self.matrix
    }

    fn get_shape(&self) -> &Shape {
        return &Shape { x: 9, y: 11 };
    }
}
