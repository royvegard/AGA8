use aga8_2017::AGA8Detail;

const GAS_002: [f64; 21] = [
    0.996_953_100, // Methane
    0.002_016_000, // Nitrogen
    0.000_093_700, // Carbon dioxide
    0.000_767_100, // Ethane
    0.000_067_900, // Propane
    0.000_019_700, // Isobutane
    0.000_006_800, // n-Butane
    0.000_015_600, // Isopentane
    0.000_000_000, // n-Pentane
    0.000_000_000, // Hexane
    0.000_000_000, // Heptane
    0.000_000_000, // Octane
    0.000_000_000, // Nonane
    0.000_000_000, // Decane
    0.000_000_000, // Hydrogen
    0.000_000_000, // Oxygen
    0.000_000_000, // Carbon monoxide
    0.000_000_000, // Water
    0.000_000_000, // Hydrogen sulfide
    0.000_060_100, // Helium
    0.000_000_000, // Argon
];

const GAS_012: [f64; 21] = [
0.981_106_02,0.008_133_99,0.001_209,0.006_111_99,0.002_153,
0.000_339,0.000_453,0.000_115,0.000_092_5,0.000_06,0.000_061,
0.000_028,0.000_003,0.000_000_5,0.0,0.0,0.0,0.0,0.0,0.000_134,0.0];

// const GAS_073: [f64; 21] = [
// 0.919_684_8,0.010_54,0.011_19,0.045_62,0.008,0.000_96,0.001_58,
// 0.000_44,0.000_36,0.000_347,0.000_238,0.000_127,0.000_033_6,
// 0.000_009_6,0.000_46,0.000_01,0.0,0.0,0.0,0.000_35,0.000_05];

// const GAS_142: [f64; 21] = [
// 0.823_150_35,0.102_460_63,0.014_761_52,0.044_363_26,0.010_047_84,
// 0.001_382_22,0.001_760_57,0.000_446_95,0.000_368_74,0.000_298_67,
// 0.000_366_16,0.000_093_33,0.000_016_39,0.000_006_12,0.000_003,
// 0.000_010_68,0.0,0.0,0.0,0.000_414_37,0.000_049_19];

// const GAS_193: [f64; 21] = [
// 0.212_401,0.077_4,0.645_369,0.005_12,0.0,0.0,0.0,0.0,0.0
// ,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.050_9,0.006_29,0.002_52];


#[test]
fn test_gas_002_01() {
    let mut aga8_test: AGA8Detail = AGA8Detail::default();
    aga8_test.setup();
    aga8_test.x = GAS_002;
    aga8_test.t = 165.933;
    aga8_test.d = 1.0;
    aga8_test.pressure_detail();
    aga8_test.properties_detail();
    assert!( f64::abs(aga8_test.p  -   1.179_2e3) < 0.1 );
    assert!( f64::abs(aga8_test.cv -  26.484_9)   < 0.000_1 );
    assert!( f64::abs(aga8_test.cp -  41.491_9)   < 0.000_1 );
    assert!( f64::abs(aga8_test.w  - 309.770_9)   < 0.000_1 );
}

#[test]
fn test_gas_002_02() {
    let mut aga8_test: AGA8Detail = AGA8Detail::default();
    aga8_test.setup();
    aga8_test.x = GAS_002;
    aga8_test.t = 169.184;
    aga8_test.d = 2.0;
    aga8_test.pressure_detail();
    aga8_test.properties_detail();
    assert!( f64::abs(aga8_test.p  -   2.050_6e3) < 0.1 );
    assert!( f64::abs(aga8_test.cv -  29.617_9)   < 0.000_1 );
    assert!( f64::abs(aga8_test.cp -  58.791)     < 0.000_1 );
    assert!( f64::abs(aga8_test.w  - 286.869)     < 0.000_1 );
}

#[test]
fn test_gas_002_03() {
    let mut aga8_test: AGA8Detail = AGA8Detail::default();
    aga8_test.setup();
    aga8_test.x = GAS_002;

    aga8_test.t = 176.24;
    aga8_test.d = 3.0;
    aga8_test.pressure_detail();
    aga8_test.properties_detail();
    assert!( f64::abs(aga8_test.p  -   2.808_8e3)  < 0.1 );
    assert!( f64::abs(aga8_test.cv -  32.196)      < 0.000_1 );
    assert!( f64::abs(aga8_test.cp -  84.340_1)    < 0.000_1 );
    assert!( f64::abs(aga8_test.w  - 275.068_5)    < 0.000_1 );
}

#[test]
fn test_gas_002_04() {
    let mut aga8_test: AGA8Detail = AGA8Detail::default();
    aga8_test.setup();
    aga8_test.x = GAS_002;
    aga8_test.t = 181.681;
    aga8_test.d = 4.0;
    aga8_test.pressure_detail();
    aga8_test.properties_detail();
    assert!( f64::abs(aga8_test.p  -    3.422_7e3)  < 0.1 );
    assert!( f64::abs(aga8_test.cv -   34.397_8)    < 0.000_1 );
    assert!( f64::abs(aga8_test.cp -  126.207)      < 0.000_1 );
    assert!( f64::abs(aga8_test.w  -  266.393_3)    < 0.000_1 );
}
#[test]
fn test_gas_002_05() {
    let x = GAS_002;
    let t = 181.681;
    let p = 40_000.0;

    let _z = aga8_2017::aga8_2017(x, p, t, 2);
    }

#[test]
fn test_gas_012_01() {
    let mut aga8_test: AGA8Detail = AGA8Detail::default();
    aga8_test.setup();
    aga8_test.x = GAS_012;
    aga8_test.t = 241.504;
    aga8_test.d = 3.54;
    aga8_test.pressure_detail();
    aga8_test.properties_detail();
    assert!( f64::abs(aga8_test.p  -    5.576_5e3)  < 0.1 );
    assert!( f64::abs(aga8_test.cv -   28.202)      < 0.000_1 );
    assert!( f64::abs(aga8_test.cp -   50.828)      < 0.000_1 );
    assert!( f64::abs(aga8_test.w  -  366.9383)     < 0.000_1 );
}

// #[test]
// fn test_gas_073_01() {
//     let mut aga8_test: AGA8Detail = AGA8Detail::default();
//     aga8_test.setup();
//     aga8_test.x = GAS_073;
//     aga8_test.t = 304.29;
//     aga8_test.d = 2.46;
//     aga8_test.pressure_detail();
//     aga8_test.properties_detail();
//     //assert!( f64::abs(aga8_test.p  -    7.586e3)  < 0.1 );
//     assert_eq!(aga8_test.p, 5.519_4e3);
//     assert!( f64::abs(aga8_test.cv -   29.2645)  < 0.000_1 );
//     assert!( f64::abs(aga8_test.cp -  45.6637)  < 0.000_1 );
//     assert!( f64::abs(aga8_test.w  -  371.1898)  < 0.000_1 );
// }

// #[test]
// fn test_gas_073_02() {
//     let mut aga8_test: AGA8Detail = AGA8Detail::default();
//     aga8_test.setup();
//     aga8_test.x = GAS_073;
//     aga8_test.t = 217.435;
//     aga8_test.d = 10.64;
//     aga8_test.pressure_detail();
//     aga8_test.properties_detail();
//     //assert!( f64::abs(aga8_test.p  -    7.586e3)  < 0.1 );
//     assert_eq!(aga8_test.p, 7.586e3);
//     assert!( f64::abs(aga8_test.cv -   28.944_2)  < 0.000_1 );
//     assert!( f64::abs(aga8_test.cp -  140.064_7)  < 0.000_1 );
//     assert!( f64::abs(aga8_test.w  -  341.345_2)  < 0.000_1 );
// }

// #[test]
// fn test_gas_142_01() {
//     let mut aga8_test: AGA8Detail = AGA8Detail::default();
//     aga8_test.setup();
//     aga8_test.x = GAS_073;
//     aga8_test.t = 369.622;
//     aga8_test.d = 1.96;
//     aga8_test.pressure_detail();
//     aga8_test.properties_detail();
//     //assert!( f64::abs(aga8_test.p  -    7.586e3)  < 0.1 );
//     assert_eq!(aga8_test.p, 5.5893e3);
//     assert!( f64::abs(aga8_test.cv -   40.7237)  < 0.000_1 );
//     assert!( f64::abs(aga8_test.cp -  53.8094)  < 0.000_1 );
//     assert!( f64::abs(aga8_test.w  -  406.7033)  < 0.000_1 );
// }
