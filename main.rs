// ============================================================
//  PRESSURIZED TANK CONTROL SYSTEM SIMULATOR
//  Algoritma dan Pemrograman - Teknik Instrumentasi
//  Bahasa: Rust | Mode: Terminal Dashboard
// ============================================================

use std::io::{self, Write};

// ============================================================
// STRUCT & IMPL (OOP Rust)
// ============================================================

// --- SENSOR (struct dasar) ---
struct Sensor {
    name: String,
    gauge_value: f64,   // nilai aktual dari gauge
    transmitter: f64,   // nilai dari transmitter
}

impl Sensor {
    fn new(name: &str) -> Sensor {
        Sensor { name: name.to_string(), gauge_value: 0.0, transmitter: 0.0 }
    }

    // Hitung error absolut antara gauge dan transmitter
    fn error(&self) -> f64 {
        (self.gauge_value - self.transmitter).abs()
    }

    // Hitung persentase error (komputasi numerik)
    fn error_pct(&self) -> f64 {
        if self.gauge_value == 0.0 { return 0.0; }
        (self.error() / self.gauge_value) * 100.0
    }

    // Validasi sensor: error < 5% dianggap valid
    fn is_valid(&self) -> bool {
        self.error_pct() < 5.0
    }

    // Linearitas transmitter: nilai terkoreksi (interpolasi linear sederhana)
    fn corrected_value(&self) -> f64 {
        // Rata-rata gauge dan transmitter sebagai nilai terkalibrasi
        (self.gauge_value + self.transmitter) / 2.0
    }
}

// --- VALVE ---
struct Valve {
    name: String,
    opening_pct: f64,  // 0.0 = tutup total, 100.0 = buka penuh
    solenoid_open: bool,
}

impl Valve {
    fn new(name: &str) -> Valve {
        Valve { name: name.to_string(), opening_pct: 50.0, solenoid_open: true }
    }

    fn status(&self) -> &str {
        if !self.solenoid_open { return "CLOSED (Emergency)"; }
        if self.opening_pct > 70.0 { "OPEN LARGE" }
        else if self.opening_pct > 30.0 { "STABLE" }
        else { "CLOSING" }
    }
}

// --- ALARM ---
struct Alarm {
    pah: f64,   // batas warning
    pahh: f64,  // batas emergency
}

impl Alarm {
    fn new(pah: f64, pahh: f64) -> Alarm {
        Alarm { pah, pahh }
    }

    fn check(&self, value: f64) -> &str {
        if value >= self.pahh { "🔴 PAHH - EMERGENCY!" }
        else if value >= self.pah { "🟡 PAH  - WARNING" }
        else { "🟢 NORMAL" }
    }
}

// --- PSV (Pressure Safety Valve) ---
struct Psv {
    open: bool,
}

impl Psv {
    fn new() -> Psv { Psv { open: false } }

    fn status(&self) -> &str {
        if self.open { "PSV: OPEN (Venting)" } else { "PSV: Closed" }
    }
}

// --- CONTROLLER (closed-loop logic) ---
struct Controller {
    temp_setpoint: f64,
    press_setpoint: f64,
}

impl Controller {
    fn new(temp_sp: f64, press_sp: f64) -> Controller {
        Controller { temp_setpoint: temp_sp, press_setpoint: press_sp }
    }

    // Hitung bukaan valve berdasarkan error suhu (P-control sederhana)
    fn calc_valve_opening(&self, actual_temp: f64) -> f64 {
        let error = self.temp_setpoint - actual_temp;
        // Gain = 2.0, valve opening proporsional terhadap error suhu
        let opening = 50.0 + error * 2.0;
        opening.clamp(0.0, 100.0)  // batas 0-100%
    }

    fn burner_status(&self, actual_temp: f64) -> &str {
        if actual_temp < self.temp_setpoint - 5.0 { "BURNER: ON  🔥" }
        else if actual_temp > self.temp_setpoint + 5.0 { "BURNER: OFF  " }
        else { "BURNER: IDLE 〰" }
    }
}

// --- TANK (agregat semua komponen) ---
struct Tank {
    temp_sensor: Sensor,
    press_sensor: Sensor,
    valve: Valve,
    alarm: Alarm,
    psv: Psv,
    controller: Controller,
}

impl Tank {
    fn new(temp_sp: f64, press_sp: f64, pah: f64, pahh: f64) -> Tank {
        Tank {
            temp_sensor: Sensor::new("Temperature"),
            press_sensor: Sensor::new("Pressure"),
            valve: Valve::new("Control Valve"),
            alarm: Alarm::new(pah, pahh),
            psv: Psv::new(),
            controller: Controller::new(temp_sp, press_sp),
        }
    }

    // Logika kontrol utama (closed-loop)
    fn update(&mut self) {
        let temp = self.temp_sensor.corrected_value();
        let press = self.press_sensor.corrected_value();

        // Hitung bukaan valve dari controller suhu
        self.valve.opening_pct = self.controller.calc_valve_opening(temp);

        // Jika tekanan tinggi, kurangi bukaan valve
        if press > self.controller.press_setpoint {
            let over = press - self.controller.press_setpoint;
            self.valve.opening_pct -= over * 3.0;
            self.valve.opening_pct = self.valve.opening_pct.clamp(0.0, 100.0);
        }

        // Emergency: PAHH → solenoid tutup, PSV buka
        let alarm_status = self.alarm.check(press);
        if alarm_status.contains("EMERGENCY") {
            self.valve.solenoid_open = false;
            self.psv.open = true;
        } else {
            self.valve.solenoid_open = true;
            self.psv.open = false;
        }
    }

    // Tampilkan dashboard terminal
    fn display_dashboard(&self) {
        let temp = self.temp_sensor.corrected_value();
        let press = self.press_sensor.corrected_value();

        println!("\n╔══════════════════════════════════════════════════╗");
        println!("║   🏭  PRESSURIZED TANK MONITORING SYSTEM  🏭     ║");
        println!("╠══════════════════════════════════════════════════╣");
        println!("║  SENSOR DATA                                     ║");
        println!("║  Temperature  : {:.2} °C  (TG:{:.2} / TT:{:.2})       ",
            temp, self.temp_sensor.gauge_value, self.temp_sensor.transmitter);
        println!("║  Pressure     : {:.2} bar (PG:{:.2} / PT:{:.2})       ",
            press, self.press_sensor.gauge_value, self.press_sensor.transmitter);
        println!("╠══════════════════════════════════════════════════╣");
        println!("║  SENSOR VALIDATION                               ║");
        println!("║  Temp  Error  : {:.2}°C ({:.2}%) → {}",
            self.temp_sensor.error(), self.temp_sensor.error_pct(),
            if self.temp_sensor.is_valid() {"✅ Valid"} else {"❌ Fault"});
        println!("║  Press Error  : {:.2}bar ({:.2}%) → {}",
            self.press_sensor.error(), self.press_sensor.error_pct(),
            if self.press_sensor.is_valid() {"✅ Valid"} else {"❌ Fault"});
        println!("╠══════════════════════════════════════════════════╣");
        println!("║  CONTROL STATUS                                  ║");
        println!("║  {}                           ", self.controller.burner_status(temp));
        println!("║  Valve Opening: {:.1}%                            ", self.valve.opening_pct);
        println!("║  Valve Status : {}                  ", self.valve.status());
        println!("║  Solenoid     : {}                  ",
            if self.valve.solenoid_open {"OPEN  ✅"} else {"CLOSED 🔴"});
        println!("╠══════════════════════════════════════════════════╣");
        println!("║  ALARM & SAFETY                                  ║");
        println!("║  Alarm Status : {}              ", self.alarm.check(press));
        println!("║  {}                         ", self.psv.status());
        println!("╚══════════════════════════════════════════════════╝");
    }
}

// ============================================================
// HELPER: baca input angka dari terminal
// ============================================================
fn input_f64(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        match s.trim().parse::<f64>() {
            Ok(v) => return v,
            Err(_) => println!("⚠ Input tidak valid, masukkan angka!"),
        }
    }
}

// ============================================================
// MAIN PROGRAM
// ============================================================
fn main() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║  PRESSURIZED TANK SIMULATOR - SETUP AWAL         ║");
    println!("╚══════════════════════════════════════════════════╝");

    // Setup awal: setpoint dan batas alarm
    let temp_sp  = input_f64("  Setpoint Suhu (°C)       : ");
    let press_sp = input_f64("  Setpoint Tekanan (bar)   : ");
    let pah      = input_f64("  Batas PAH  (bar)         : ");
    let pahh     = input_f64("  Batas PAHH (bar)         : ");

    // Buat objek tank
    let mut tank = Tank::new(temp_sp, press_sp, pah, pahh);

    // Loop monitoring utama
    loop {
        println!("\n──────────────────────────────────────────────────");
        println!("  MASUKKAN DATA SENSOR (ketik 'q' untuk keluar)");
        println!("──────────────────────────────────────────────────");

        // Input data sensor
        tank.temp_sensor.gauge_value  = input_f64("  Temperature Gauge  TG (°C)  : ");
        tank.temp_sensor.transmitter  = input_f64("  Temperature Transmitter TT  : ");
        tank.press_sensor.gauge_value = input_f64("  Pressure Gauge     PG (bar) : ");
        tank.press_sensor.transmitter = input_f64("  Pressure Transmitter   PT   : ");

        // Jalankan logika kontrol
        tank.update();

        // Tampilkan dashboard
        tank.display_dashboard();

        // Tanya lanjut atau tidak
        print!("\n  Lanjut monitoring? (y/n): ");
        io::stdout().flush().unwrap();
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();
        if ans.trim().to_lowercase() != "y" {
            println!("\n  Sistem dimatikan. Terima kasih!\n");
            break;
        }
    }
}
