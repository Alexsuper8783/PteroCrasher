use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use rand::Rng;

fn help() {
    print!("\x1B[0;34m");
    print!(r#"
                                    |        |Pterodactyl Crasher|      |
                                    |            By Alexsuper           |
                                    |               1.0.0               |

                                                    |Args|

crasher (-h | --help) - Printing this help message
crasher (-m | --mode) (cpu | ram) - Selecting mode cpu or ram
crasher (-t | --threads) (number: u32) - Number of threads to work
crasher (-s | --sleep) (time: u32) - milliseconds to sleep after allocation or calculation
crasher (-p | --power) (power: u128) - (mode - ram power is allocattion size in mb | mode - cpu power is difficulty of calculation )
"#)
}

fn heavy_math_calculation(input: u128) -> u128 {
    let mut x = input;
    for _ in 0..100 {
        x = (x ^ (x << 3)).wrapping_mul(x.wrapping_add(0xDEADBEEF));
        x = x.wrapping_pow(3) ^ (x >> 5);

        let f = x as f64 * std::f64::consts::PI / 1_000_000_000.0;
        x = x.wrapping_add(f.sin().abs() as u128 * 2);
    }
    x
}

fn memory_eater(number: u32, power: u128, sleep: u32) {
    println!("Memory eater number {} activated", number);
    let mut map = HashMap::new();
    let mut counter: u128 = 0;

    loop {
        let key = format!("mem_block_{}", counter);
        map.insert(key, vec![counter as u8; (power * 1024 * 1024) as usize]);
        counter += 1;

        println!("Allocated: {} MB", map.len() * power as usize);
        thread::sleep(Duration::from_millis(sleep as u64));
    }
}

fn cpu_eater(number: u32, power: u128, sleep: u32) {
    println!("CPU eater number {}", number);
    let mut rng = rand::rng();
    loop {
        for _i in 0..=power {
            let number = rng.random::<u128>();

            let result = heavy_math_calculation(number);

            if result == 0 {
                unreachable!()
            }
        }
        thread::sleep(Duration::from_millis(sleep as u64));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut mode: bool = true;
    let mut threads: u32 = 1;
    let mut sleep: u32 = 100;
    let mut power: u128 = 1;

    if args.len() <= 2 {
        help();
        return;
    } else {
        for i in 1..args.len() {
            match args[i].as_str() {
                "-t" | "--threads" => {
                    if i + 1 <= args.len() {
                        threads = args[i + 1].parse().expect("Invalid number");
                    } else {
                        help();
                        return;
                    }
                }
                "-s" | "--sleep" => {
                    if i + 1 <= args.len() {
                        sleep = args[i + 1].parse().expect("Invalid number");
                    } else {
                        help();
                        return;
                    }
                }
                "-p" | "--power" => {
                    if i + 1 <= args.len() {
                        power = args[i + 1].parse().expect("Invalid number");
                    } else {
                        help();
                        return;
                    }
                }
                "-m" | "--mode" => {
                    if i + 1 <= args.len() {
                        if args[i + 1] == "cpu" {
                            mode = false;
                        } else if args[i + 1] == "ram" {
                            mode = true;
                        } else {
                            help();
                            return;
                        }
                    }
                }
                "-h" | "--help" => {
                    help();
                    return;
                }
                _ => {}
            }
        }
    }

    if mode {
        for thread in 0..threads {
            thread::spawn(move || {
                memory_eater(thread, power, sleep);
            });
        }
        thread::park();
    }
    if !mode {
        for thread in 0..threads {
            thread::spawn(move || {
                cpu_eater(thread, power, sleep);
            });
        }
        thread::park();
    }
}
