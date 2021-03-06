use defmt::Format;

/// Holds the State for the application.
#[derive(Format)]
pub struct State {
    current_boiler_temp: f32,
    target_boiler_temp: f32,
    last_pid_out: f32,
    heater_on: bool,
    kp: f32,
    ki: f32,
    kd: f32,
    coldstart: bool,
    watchdog_reset: bool,
}

impl State {
    pub fn new(
        target_boiler_temp: f32,
        heater_on: bool,
        kp: f32,
        ki: f32,
        kd: f32,
        coldstart: bool,
        watchdog_reset: bool,
    ) -> Self {
        Self {
            current_boiler_temp: 0.0,
            target_boiler_temp,
            last_pid_out: 0.0,
            heater_on,
            kp,
            ki,
            kd,
            coldstart,
            watchdog_reset,
        }
    }

    pub fn set_current_boiler_temp(&mut self, current_boiler_temp: f32) {
        self.current_boiler_temp = current_boiler_temp;
    }

    pub fn set_last_pid_out(&mut self, pid_out: f32) {
        self.last_pid_out = pid_out;
    }

    pub fn current_boiler_temp(&self) -> f32 {
        self.current_boiler_temp
    }

    pub fn set_target_boiler_temp(&mut self, target_boiler_temp: f32) {
        self.target_boiler_temp = target_boiler_temp;
    }

    pub fn target_boiler_temp(&self) -> f32 {
        self.target_boiler_temp
    }

    pub fn set_heater_on(&mut self, heater_on: bool) {
        self.heater_on = heater_on;
    }

    pub fn set_watchdog_reset(&mut self, reset: bool) {
        self.watchdog_reset = reset;
    }

    pub fn watchdog_reset(&self) -> bool {
        self.watchdog_reset
    }

    pub fn heater_on(&self) -> bool {
        self.heater_on
    }

    pub fn kp(&self) -> f32 {
        self.kp
    }
    pub fn ki(&self) -> f32 {
        self.ki
    }
    pub fn kd(&self) -> f32 {
        self.kd
    }

    pub fn set_kp(&mut self, kp: f32) {
        self.kp = kp;
    }
    pub fn set_ki(&mut self, ki: f32) {
        self.ki = ki;
    }
    pub fn set_kd(&mut self, kd: f32) {
        self.kd = kd;
    }

    pub fn in_coldstart(&self) -> bool {
        self.coldstart
    }

    pub fn disable_coldstart(&mut self) {
        self.coldstart = false;
    }

    pub fn last_pid_out(&self) -> f32 {
        self.last_pid_out
    }
}
