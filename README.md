# Sistem-Kontrol-Suhu-dan-Tekanan-pada-Pressurerized-Tank-
 program ini mensimulasikan sistem kontrol pada sebuah tangki bertekanan (pressurized tank) . User berperan sebagai operator yang mengatur setpoint, menginput nilai sensor secara manual lewat terminal, lalu program akan otomatis menghitung, memvalidasi, dan menampilkan status sistem dalam bentuk dashboard sederhana.
## Yang Bisa Dilakukan Program Ini
- Mengatur Setpoint suhu dan tekanan serta batas PAH dan PAHH 
- Input nilai sensor suhu dan tekanan (TG, TT, PG, PT) secara manual
- Mengvalidasi sensor, jika error antar gauge dan transmitter lebih dari 5%, dianggap fault
- Jika sensor fault, otomatis pakai nilai gauge fisik sebagai fallback
- Kontrol bukaan valve secara otomatis berdasarkan suhu
- Nyalain alarm kalau tekanan mendekati batas bahaya (PAH) 
- Emergency shutdown otomatis kalau tekanan melewati batas kritis (PAHH)
- Tampilkan semua status dalam dashboard terminal
## Tampilan Dashboard
 
```
══════════════════════════════════════════
 🏭  PRESSURIZED TANK MONITORING SYSTEM  
══════════════════════════════════════════
  Temp  : 102.00°C  (TG:101.00 / TT:103.00) [avg]
  Press : 3.05 bar (PG:3.00 / PT:3.10) [avg]
══════════════════════════════════════════
 Temp  Error : 2.00°C (1.98%) → ✅ Valid
  Press Error : 0.10bar (3.33%) → ✅ Valid
══════════════════════════════════════════
  BURNER: IDLE 〰
  Valve   : 46.0% — STABLE
  Solenoid: OPEN ✅
══════════════════════════════════════════
  Alarm : 🟢 NORMAL
  PSV   : Closed ✅
══════════════════════════════════════════
```
 
---
 
## Cara Jalankan
 
Pastikan [Rust](https://rustup.rs) sudah terinstall, lalu:
 
```bash
git clone https://github.com/username/tank-simulator.git
cd tank-simulator
cargo run
```
 
---
 
## Dibuat Dengan
 
- Bahasa: **Rust**
- Konsep: OOP (struct & impl), closed-loop control, sensor validation, P-control
- Tools: VS Code + rust-analyzer extension
