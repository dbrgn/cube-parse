use std::{collections::HashMap, env, fs::File, path::Path};

use alphanumeric_sort::compare_str;
use serde_xml_rs;

mod mcu;
mod internal_peripheral;

fn render_pin_modes(ip: &internal_peripheral::IP) {
    let mut pin_map: HashMap<String, Vec<String>> = HashMap::new();

    for p in &ip.gpio_pin {
        let name = p.get_name();
        if let Some(n) = name {
            pin_map.insert(n, p.get_af_modes());
        }
    }

    let mut pin_map = pin_map
        .into_iter()
        .map(|(k, mut v)| {
            #[allow(clippy::redundant_closure)]
            v.sort_by(|a, b| compare_str(a, b));
            (k, v)
        })
        .collect::<Vec<_>>();

    pin_map.sort_by(|a, b| compare_str(&a.0, &b.0));

    println!("pins! {{");
    for (n, af) in pin_map {
        if af.is_empty() {
            continue;
        } else if af.len() == 1 {
            println!("    {} => {{{}}},", n, af[0]);
        } else {
            println!("    {} => {{", n);
            for a in af {
                println!("        {},", a);
            }
            println!("    }},");
        }
    }
    println!("}}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: ./cube-parse CUBEMX_MCU_DB_DIR MCU_FILE")
    }

    let db_dir = Path::new(&args[1]);

    let val: mcu::Mcu = {
        let mcu_file = db_dir.with_file_name(format!("{}.xml", &args[2]));

        let mut fin = File::open(&mcu_file).unwrap();

        serde_xml_rs::deserialize(&mut fin).unwrap()
    };

    let gpio: internal_peripheral::IP = {
        let gpio_name = val.get_ip("GPIO").unwrap().get_version();

        let gpio_file = db_dir.with_file_name(format!("IP/GPIO-{}_Modes.xml", gpio_name));

        let mut fin = File::open(&gpio_file).unwrap();

        serde_xml_rs::deserialize(&mut fin).unwrap()
    };

    render_pin_modes(&gpio);
}