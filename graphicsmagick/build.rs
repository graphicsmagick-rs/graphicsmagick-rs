#![allow(clippy::unreadable_literal)]

fn main() -> anyhow::Result<()> {
    let version_map = [
        ("gm_v_1_3_35", 0x242100),
        ("gm_v_1_3_34", 0x232002),
        ("gm_v_1_3_33", 0x232001),
        ("gm_v_1_3_32", 0x232000),
        ("gm_v_1_3_31", 0x221900),
        ("gm_v_1_3_30", 0x211801),
        ("gm_v_1_3_29", 0x211800),
        ("gm_v_1_3_28", 0x201702),
        ("gm_v_1_3_27", 0x201700),
        ("gm_v_1_3_26", 0x191600),
        ("gm_v_1_3_25", 0x181501),
        ("gm_v_1_3_24", 0x181500),
        ("gm_v_1_3_23", 0x171401),
        ("gm_v_1_3_22", 0x171400),
        ("gm_v_1_3_21", 0x161300),
        ("gm_v_1_3_20", 0x151200),
    ];

    for (version, lib_version) in version_map.iter() {
        if graphicsmagick_sys::MagickLibVersion >= *lib_version {
            println!("cargo:rustc-cfg={}", version);
        }
    }

    Ok(())
}
