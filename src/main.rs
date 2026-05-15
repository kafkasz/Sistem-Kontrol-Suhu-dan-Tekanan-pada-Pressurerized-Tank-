// PRESSURIZED TANK CONTROL SYSTEM SIMULATOR
// ETS Algoritma dan Pemrograman - Teknik Instrumentasi
use std::io::{self, Write};

// ── STRUCT ──────────────────────────────────────────────────
struct Sensor { gauge: f64, tt: f64 }
struct Valve  { opening: f64, solenoid: bool }
struct Alarm  { pah: f64, pahh: f64 }
struct Psv    { open: bool }
struct Controller { sp_temp: f64, sp_press: f64 }

struct Tank {
    temp:  Sensor,
    press: Sensor,
    valve: Valve,
    alarm: Alarm,
    psv:   Psv,
    ctrl:  Controller,
}

// ── IMPL ─────────────────────────────────────────────────────
impl Sensor {
    fn new() -> Self { Sensor { gauge: 0.0, tt: 0.0 } }
    // Sensor fallback: valid → rata-rata (akurat), fault → PG saja
    fn value(&self) -> f64 {
        if self.valid() { (self.gauge + self.tt) / 2.0 }
        else            { self.gauge }
    }
    fn error(&self)     -> f64  { (self.gauge - self.tt).abs() }          // error absolut
    fn error_pct(&self) -> f64  { if self.gauge == 0.0 { 0.0 } else { self.error() / self.gauge * 100.0 } }
    fn valid(&self)     -> bool { self.error_pct() < 5.0 }
}

impl Valve {
    fn new() -> Self { Valve { opening: 50.0, solenoid: true } }
    fn status(&self) -> &str {
        if !self.solenoid       { "CLOSED (Emergency)" }
        else if self.opening > 70.0 { "OPEN LARGE" }
        else if self.opening > 30.0 { "STABLE" }
        else                    { "CLOSING" }
    }
}

impl Alarm {
    fn check(&self, p: f64) -> &str {
        if p >= self.pahh { "🔴 PAHH - EMERGENCY!" }
        else if p >= self.pah { "🟡 PAH  - WARNING" }
        else { "🟢 NORMAL" }
    }
}

impl Controller {
    // P-control: bukaan valve proporsional thd error suhu (gain=2)
    fn valve_opening(&self, t: f64) -> f64 {
        (50.0 + (self.sp_temp - t) * 2.0).clamp(0.0, 100.0)
    }
    fn burner(&self, t: f64) -> &str {
        if t < self.sp_temp - 5.0 { "BURNER: ON  🔥" }
        else if t > self.sp_temp + 5.0 { "BURNER: OFF" }
        else { "BURNER: IDLE 〰" }
    }
}

impl Tank {
    fn new(sp_temp: f64, sp_press: f64, pah: f64, pahh: f64) -> Self {
        Tank {
            temp:  Sensor::new(), press: Sensor::new(),
            valve: Valve::new(),
            alarm: Alarm { pah, pahh },
            psv:   Psv { open: false },
            ctrl:  Controller { sp_temp, sp_press },
        }
    }

    // Closed-loop control logic
    fn update(&mut self) {
        let t = self.temp.value();
        let p = self.press.value();

        // Hitung bukaan valve dari suhu, lalu koreksi dari tekanan
        self.valve.opening = self.ctrl.valve_opening(t);
        if p > self.ctrl.sp_press {
            self.valve.opening -= (p - self.ctrl.sp_press) * 3.0;
            self.valve.opening  = self.valve.opening.clamp(0.0, 100.0);
        }

        // Emergency shutdown jika PAHH tercapai
        let emergency = self.alarm.check(p).contains("EMERGENCY");
        self.valve.solenoid = !emergency;
        self.psv.open       =  emergency;
    }

    fn dashboard(&self) {
        let t = self.temp.value();
        let p = self.press.value();
        println!("\n══════════════════════════════════════════");
        println!(" 🏭  PRESSURIZED TANK MONITORING SYSTEM  ");
        println!("══════════════════════════════════════════");
        println!("   Temp  : {:.2}°C  (TG:{:.2} / TT:{:.2}) [{}]", t, self.temp.gauge, self.temp.tt,
            if self.temp.valid() {"avg"} else {"PG"});
        println!("  Press : {:.2} bar (PG:{:.2} / PT:{:.2}) [{}]", p, self.press.gauge, self.press.tt,
            if self.press.valid() {"avg"} else {"PG"});
        println!("══════════════════════════════════════════");
        println!(" Temp  Error : {:.2}°C ({:.2}%) → {}",
            self.temp.error(), self.temp.error_pct(),
            if self.temp.valid() {"✅ Valid"} else {"❌ Fault"});
        println!("  Press Error : {:.2}bar ({:.2}%) → {}",
            self.press.error(), self.press.error_pct(),
            if self.press.valid() {"✅ Valid"} else {"❌ Fault"});
        println!("══════════════════════════════════════════");
        println!("  {}",       self.ctrl.burner(t));
        println!("  Valve   : {:.1}% — {}", self.valve.opening, self.valve.status());
        println!("  Solenoid: {}", if self.valve.solenoid {"OPEN ✅"} else {"CLOSED 🔴"});
        println!("══════════════════════════════════════════");
        println!("  Alarm : {}", self.alarm.check(p));
        println!("  PSV   : {}", if self.psv.open {"OPEN (Venting) 🔴"} else {"Closed ✅"});
        println!("══════════════════════════════════════════");
    }
}

// ── HELPER INPUT ─────────────────────────────────────────────
fn input(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(v) = s.trim().parse::<f64>() { return v; }
        println!("⚠ Masukkan angka yang valid!");
    }
}

// ── MAIN ─────────────────────────────────────────────────────
fn main() {
    println!("=== PRESSURIZED TANK SIMULATOR — SETUP ===");
    let mut tank = Tank::new(
        input("Setpoint Suhu    (°C) : "),
        input("Setpoint Tekanan (bar): "),
        input("Batas PAH        (bar): "),
        input("Batas PAHH       (bar): "),
    );

    loop {
        println!("\n--- Input Sensor ---");
        tank.temp.gauge  = input("TG  - Temperature Gauge (°C) : ");
        tank.temp.tt     = input("TT  - Temperature Transmitter: ");
        tank.press.gauge = input("PG  - Pressure Gauge    (bar): ");
        tank.press.tt    = input("PT  - Pressure Transmitter   : ");

        tank.update();
        tank.dashboard();

        print!("\nLanjut? (y/n): ");
        io::stdout().flush().unwrap();
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();
        if ans.trim() != "y" { println!("Sistem dimatikan."); break; }
    }
}