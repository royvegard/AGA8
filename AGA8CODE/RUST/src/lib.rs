mod detail;
mod gerg2008;

pub use crate::detail::AGA8Detail;
pub use crate::gerg2008::Gerg2008;

use std::slice;

#[repr(C)]
pub struct Result {
    pub d: f64, // Molar concentration [mol/l]
    pub mm: f64,
    pub z: f64,
    pub dp_dd: f64,
    pub d2p_dd2: f64,
    pub dp_dt: f64,
    pub u: f64,
    pub h: f64,
    pub s: f64,
    pub cv: f64,
    pub cp: f64,
    pub w: f64,
    pub g: f64,
    pub jt: f64,
    pub kappa: f64,
}

/// # Safety
/// composition must be an array of 21 elements.
#[no_mangle]
pub unsafe extern "C" fn aga8_2017(
    composition: *const f64,
    pressure: f64,
    temperature: f64,
) -> Result {
    let array = {
        assert!(!composition.is_null());
        slice::from_raw_parts(composition, detail::NC_DETAIL)
    };

    let mut aga8_test: AGA8Detail = AGA8Detail::new();
    aga8_test.setup();

    aga8_test.x[0..detail::NC_DETAIL].clone_from_slice(&array[..]);

    aga8_test.t = temperature;
    aga8_test.p = pressure;
    aga8_test.density_detail();
    aga8_test.properties_detail();

    Result {
        d: aga8_test.d, // Molar concentration [mol/l]
        mm: aga8_test.mm,
        z: aga8_test.z,
        dp_dd: aga8_test.dp_dd,
        d2p_dd2: aga8_test.d2p_dd2,
        dp_dt: aga8_test.dp_dt,
        u: aga8_test.u,
        h: aga8_test.h,
        s: aga8_test.s,
        cv: aga8_test.cv,
        cp: aga8_test.cp,
        w: aga8_test.w,
        g: aga8_test.g,
        jt: aga8_test.jt,
        kappa: aga8_test.kappa,
    }
}

#[no_mangle]
pub extern "C" fn aga8_new() -> *mut AGA8Detail {
    Box::into_raw(Box::new(AGA8Detail::new()))
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn aga8_free(ptr: *mut AGA8Detail) {
    if ptr.is_null() {
        return;
    }
    Box::from_raw(ptr);
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn aga8_setup(ptr: *mut AGA8Detail) {
    assert!(!ptr.is_null());
    let aga8 = &mut *ptr;
    aga8.setup();
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn aga8_set_composition(ptr: *mut AGA8Detail, composition: *const f64) {
    assert!(!ptr.is_null());
    assert!(!composition.is_null());
    let aga8 = &mut *ptr;
    let array = slice::from_raw_parts(composition, detail::NC_DETAIL);
    aga8.x[0..detail::NC_DETAIL].clone_from_slice(&array[..]);
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn aga8_set_pressure(ptr: *mut AGA8Detail, pressure: f64) {
    assert!(!ptr.is_null());
    let aga8 = &mut *ptr;
    aga8.p = pressure;
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn aga8_get_pressure(ptr: *mut AGA8Detail) -> f64 {
    assert!(!ptr.is_null());
    let aga8 = &mut *ptr;
    aga8.p
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn aga8_set_temperature(ptr: *mut AGA8Detail, temperature: f64) {
    assert!(!ptr.is_null());
    let aga8 = &mut *ptr;
    aga8.t = temperature;
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn aga8_get_temperature(ptr: *mut AGA8Detail) -> f64 {
    assert!(!ptr.is_null());
    let aga8 = &mut *ptr;
    aga8.t
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn aga8_set_density(ptr: *mut AGA8Detail, density: f64) {
    assert!(!ptr.is_null());
    let aga8 = &mut *ptr;
    aga8.d = density;
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn aga8_get_density(ptr: *mut AGA8Detail) -> f64 {
    assert!(!ptr.is_null());
    let aga8 = &mut *ptr;
    aga8.d
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn aga8_get_properties(ptr: *const AGA8Detail) -> Result {
    assert!(!ptr.is_null());
    let aga8 = &*ptr;
    Result {
        d: aga8.d, // Molar concentration [mol/l]
        mm: aga8.mm,
        z: aga8.z,
        dp_dd: aga8.dp_dd,
        d2p_dd2: aga8.d2p_dd2,
        dp_dt: aga8.dp_dt,
        u: aga8.u,
        h: aga8.h,
        s: aga8.s,
        cv: aga8.cv,
        cp: aga8.cp,
        w: aga8.w,
        g: aga8.g,
        jt: aga8.jt,
        kappa: aga8.kappa,
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn aga8_calculate_pressure(ptr: *mut AGA8Detail) {
    assert!(!ptr.is_null());
    let aga8 = &mut *ptr;
    aga8.pressure_detail();
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn aga8_calculate_density(ptr: *mut AGA8Detail) {
    assert!(!ptr.is_null());
    let aga8 = &mut *ptr;
    aga8.density_detail();
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn aga8_calculate_properties(ptr: *mut AGA8Detail) {
    assert!(!ptr.is_null());
    let aga8 = &mut *ptr;
    aga8.properties_detail();
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn gerg_2008(
    composition: *const f64,
    pressure: f64,
    temperature: f64,
) -> Result {
    assert!(!composition.is_null());
    let array = slice::from_raw_parts(composition, detail::NC_DETAIL);

    let mut gerg_test: Gerg2008 = Gerg2008::new();
    gerg_test.setup();

    gerg_test.x[1..=detail::NC_DETAIL].clone_from_slice(&array[..]);
    gerg_test.t = temperature;
    gerg_test.p = pressure;
    gerg_test.density(0);
    gerg_test.properties();

    Result {
        d: gerg_test.d, // Molar concentration [mol/l]
        mm: gerg_test.mm,
        z: gerg_test.z,
        dp_dd: gerg_test.dp_dd,
        d2p_dd2: gerg_test.d2p_dd2,
        dp_dt: gerg_test.dp_dt,
        u: gerg_test.u,
        h: gerg_test.h,
        s: gerg_test.s,
        cv: gerg_test.cv,
        cp: gerg_test.cp,
        w: gerg_test.w,
        g: gerg_test.g,
        jt: gerg_test.jt,
        kappa: gerg_test.kappa,
    }
}

#[no_mangle]
pub extern "C" fn gerg_new() -> *mut Gerg2008 {
    Box::into_raw(Box::new(Gerg2008::new()))
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn gerg_free(ptr: *mut Gerg2008) {
    if ptr.is_null() {
        return;
    }
    Box::from_raw(ptr);
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn gerg_setup(ptr: *mut Gerg2008) {
    assert!(!ptr.is_null());
    let gerg = &mut *ptr;
    gerg.setup();
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn gerg_set_composition(ptr: *mut Gerg2008, composition: *const f64) {
    assert!(!ptr.is_null());
    assert!(!composition.is_null());
    let gerg = &mut *ptr;
    let array = slice::from_raw_parts(composition, detail::NC_DETAIL);
    gerg.x[1..=detail::NC_DETAIL].clone_from_slice(&array[..]);
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn gerg_set_pressure(ptr: *mut Gerg2008, pressure: f64) {
    assert!(!ptr.is_null());
    let gerg = &mut *ptr;
    gerg.p = pressure;
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn gerg_get_pressure(ptr: *mut Gerg2008) -> f64 {
    assert!(!ptr.is_null());
    let gerg = &mut *ptr;
    gerg.p
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn gerg_set_temperature(ptr: *mut Gerg2008, temperature: f64) {
    assert!(!ptr.is_null());
    let gerg = &mut *ptr;
    gerg.t = temperature;
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn gerg_get_temperature(ptr: *mut Gerg2008) -> f64 {
    assert!(!ptr.is_null());
    let gerg = &mut *ptr;
    gerg.t
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn gerg_set_density(ptr: *mut Gerg2008, density: f64) {
    assert!(!ptr.is_null());
    let gerg = &mut *ptr;
    gerg.d = density;
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn gerg_get_density(ptr: *mut Gerg2008) -> f64 {
    assert!(!ptr.is_null());
    let gerg = &mut *ptr;
    gerg.d
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn gerg_get_properties(ptr: *const Gerg2008) -> Result {
    assert!(!ptr.is_null());
    let gerg = &*ptr;
    Result {
        d: gerg.d, // Molar concentration [mol/l]
        mm: gerg.mm,
        z: gerg.z,
        dp_dd: gerg.dp_dd,
        d2p_dd2: gerg.d2p_dd2,
        dp_dt: gerg.dp_dt,
        u: gerg.u,
        h: gerg.h,
        s: gerg.s,
        cv: gerg.cv,
        cp: gerg.cp,
        w: gerg.w,
        g: gerg.g,
        jt: gerg.jt,
        kappa: gerg.kappa,
    }
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn gerg_calculate_pressure(ptr: *mut Gerg2008) {
    assert!(!ptr.is_null());
    let gerg = &mut *ptr;
    gerg.pressure();
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn gerg_calculate_density(ptr: *mut Gerg2008) {
    assert!(!ptr.is_null());
    let gerg = &mut *ptr;
    gerg.density(0);
}

/// # Safety
///
#[no_mangle]
pub unsafe extern "C" fn gerg_calculate_properties(ptr: *mut Gerg2008) {
    assert!(!ptr.is_null());
    let gerg = &mut *ptr;
    gerg.properties();
}
