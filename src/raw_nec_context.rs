use crate::bindings;

use std::{
    ffi::{CStr, CString},
    os::raw::{c_int, c_long},
    path::Path,
};

pub fn nec_error_message() -> String {
    unsafe { CStr::from_ptr(bindings::nec_error_message()) }
        .to_string_lossy()
        .to_owned()
        .to_string()
}

pub fn check_result(result: c_long) {
    match result {
        0 => {}
        _ => {
            panic!("{}", nec_error_message())
        }
    }
}

pub struct RawNecContext {
    ptr: *mut bindings::nec_context,
}

impl Drop for RawNecContext {
    fn drop(&mut self) {
        check_result(unsafe { bindings::nec_delete(self.ptr) });
    }
}

impl RawNecContext {
    pub fn new() -> Self {
        Self {
            ptr: unsafe { bindings::nec_create() },
        }
    }

    pub fn nec_wire(
        &mut self,
        tag_id: ::std::os::raw::c_int,
        segment_count: ::std::os::raw::c_int,
        xw1: f64,
        yw1: f64,
        zw1: f64,
        xw2: f64,
        yw2: f64,
        zw2: f64,
        rad: f64,
        rdel: f64,
        rrad: f64,
    ) {
        eprintln!(
            "WIRE {} {} {} {} {} {} {} {} {} {} {}",
            tag_id, segment_count, xw1, yw1, zw1, xw2, yw2, zw2, rad, rdel, rrad
        );
        check_result(unsafe {
            bindings::nec_wire(
                self.ptr,
                tag_id,
                segment_count,
                xw1,
                yw1,
                zw1,
                xw2,
                yw2,
                zw2,
                rad,
                rdel,
                rrad,
            )
        })
    }

    pub fn nec_sp_card(
        &mut self,
        ns: ::std::os::raw::c_int,
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
    ) {
        check_result(unsafe { bindings::nec_sp_card(self.ptr, ns, x1, y1, z1, x2, y2, z2) })
    }

    pub fn nec_sc_card(
        &mut self,
        i2: ::std::os::raw::c_int,
        x3: f64,
        y3: f64,
        z3: f64,
        x4: f64,
        y4: f64,
        z4: f64,
    ) {
        check_result(unsafe { bindings::nec_sc_card(self.ptr, i2, x3, y3, z3, x4, y4, z4) })
    }

    pub fn nec_gm_card(
        &mut self,
        itsi: ::std::os::raw::c_int,
        nrpt: ::std::os::raw::c_int,
        rox: f64,
        roy: f64,
        roz: f64,
        xs: f64,
        ys: f64,
        zs: f64,
        its: ::std::os::raw::c_int,
    ) {
        eprintln!(
            "GM {} {} {} {} {} {} {} {} {}",
            itsi, nrpt, rox, roy, roz, xs, ys, zs, its
        );
        check_result(unsafe {
            bindings::nec_gm_card(self.ptr, itsi, nrpt, rox, roy, roz, xs, ys, zs, its)
        })
    }

    pub fn nec_gx_card(&mut self, i1: ::std::os::raw::c_int, i2: ::std::os::raw::c_int) {
        check_result(unsafe { bindings::nec_gx_card(self.ptr, i1, i2) });
    }

    pub fn nec_geometry_complete(&mut self, gpflag: ::std::os::raw::c_int) {
        eprintln!("geom completed");
        check_result(unsafe { bindings::nec_geometry_complete(self.ptr, gpflag) })
    }

    pub fn nec_medium_parameters(&mut self, permittivity: f64, permeability: f64) {
        check_result(unsafe {
            bindings::nec_medium_parameters(self.ptr, permittivity, permeability)
        })
    }

    pub fn nec_gn_card(
        &mut self,
        iperf: ::std::os::raw::c_int,
        nradl: ::std::os::raw::c_int,
        epse: f64,
        sig: f64,
        tmp3: f64,
        tmp4: f64,
        tmp5: f64,
        tmp6: f64,
    ) {
        eprintln!(
            "GN {} {} {} {} {} {} {} {}",
            iperf, nradl, epse, sig, tmp3, tmp4, tmp5, tmp6
        );
        check_result(unsafe {
            bindings::nec_gn_card(self.ptr, iperf, nradl, epse, sig, tmp3, tmp4, tmp5, tmp6)
        })
    }

    pub fn nec_fr_card(
        &mut self,
        in_ifrq: ::std::os::raw::c_int,
        in_nfrq: ::std::os::raw::c_int,
        in_freq_mhz: f64,
        in_del_freq: f64,
    ) {
        eprintln!("FR {} {} {} {}", in_ifrq, in_nfrq, in_freq_mhz, in_del_freq);
        check_result(unsafe {
            bindings::nec_fr_card(self.ptr, in_ifrq, in_nfrq, in_freq_mhz, in_del_freq)
        })
    }

    pub fn nec_ek_card(&mut self, itmp1: ::std::os::raw::c_int) {
        check_result(unsafe { bindings::nec_ek_card(self.ptr, itmp1) })
    }

    pub fn nec_ld_card(
        &mut self,
        ldtyp: ::std::os::raw::c_int,
        ldtag: ::std::os::raw::c_int,
        ldtagf: ::std::os::raw::c_int,
        ldtagt: ::std::os::raw::c_int,
        tmp1: f64,
        tmp2: f64,
        tmp3: f64,
    ) {
        check_result(unsafe {
            bindings::nec_ld_card(self.ptr, ldtyp, ldtag, ldtagf, ldtagt, tmp1, tmp2, tmp3)
        })
    }

    pub fn nec_ex_card(
        &mut self,
        extype: ::std::os::raw::c_int,
        i2: ::std::os::raw::c_int,
        i3: ::std::os::raw::c_int,
        i4: ::std::os::raw::c_int,
        tmp1: f64,
        tmp2: f64,
        tmp3: f64,
        tmp4: f64,
        tmp5: f64,
        tmp6: f64,
    ) {
        eprintln!(
            "EX {} {} {} {} {} {} {} {} {} {}",
            extype, i2, i3, i4, tmp1, tmp2, tmp3, tmp4, tmp5, tmp6
        );
        check_result(unsafe {
            bindings::nec_ex_card(
                self.ptr, extype, i2, i3, i4, tmp1, tmp2, tmp3, tmp4, tmp5, tmp6,
            )
        })
    }

    pub fn nec_excitation_voltage(
        &mut self,
        tag: ::std::os::raw::c_int,
        segment: ::std::os::raw::c_int,
        v_real: f64,
        v_imag: f64,
    ) {
        check_result(unsafe {
            bindings::nec_excitation_voltage(self.ptr, tag, segment, v_real, v_imag)
        })
    }

    pub fn nec_excitation_current(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        a: f64,
        beta: f64,
        moment: f64,
    ) {
        check_result(unsafe {
            bindings::nec_excitation_current(self.ptr, x, y, z, a, beta, moment)
        })
    }

    pub fn nec_excitation_planewave(
        &mut self,
        n_theta: ::std::os::raw::c_int,
        n_phi: ::std::os::raw::c_int,
        theta: f64,
        phi: f64,
        eta: f64,
        dtheta: f64,
        dphi: f64,
        pol_ratio: f64,
    ) {
        check_result(unsafe {
            bindings::nec_excitation_planewave(
                self.ptr, n_theta, n_phi, theta, phi, eta, dtheta, dphi, pol_ratio,
            )
        })
    }

    pub fn nec_tl_card(
        &mut self,
        itmp1: ::std::os::raw::c_int,
        itmp2: ::std::os::raw::c_int,
        itmp3: ::std::os::raw::c_int,
        itmp4: ::std::os::raw::c_int,
        tmp1: f64,
        tmp2: f64,
        tmp3: f64,
        tmp4: f64,
        tmp5: f64,
        tmp6: f64,
    ) {
        eprintln!(
            "TL {} {} {} {} {} {} {} {} {} {}",
            itmp1, itmp2, itmp3, itmp4, tmp1, tmp2, tmp3, tmp4, tmp5, tmp6
        );
        check_result(unsafe {
            bindings::nec_tl_card(
                self.ptr, itmp1, itmp2, itmp3, itmp4, tmp1, tmp2, tmp3, tmp4, tmp5, tmp6,
            )
        })
    }

    pub fn nec_nt_card(
        &mut self,
        itmp1: ::std::os::raw::c_int,
        itmp2: ::std::os::raw::c_int,
        itmp3: ::std::os::raw::c_int,
        itmp4: ::std::os::raw::c_int,
        tmp1: f64,
        tmp2: f64,
        tmp3: f64,
        tmp4: f64,
        tmp5: f64,
        tmp6: f64,
    ) {
        check_result(unsafe {
            bindings::nec_nt_card(
                self.ptr, itmp1, itmp2, itmp3, itmp4, tmp1, tmp2, tmp3, tmp4, tmp5, tmp6,
            )
        })
    }

    pub fn nec_xq_card(&mut self, itmp1: ::std::os::raw::c_int) {
        check_result(unsafe { bindings::nec_xq_card(self.ptr, itmp1) })
    }

    pub fn nec_gd_card(&mut self, tmp1: f64, tmp2: f64, tmp3: f64, tmp4: f64) {
        check_result(unsafe { bindings::nec_gd_card(self.ptr, tmp1, tmp2, tmp3, tmp4) })
    }

    pub fn nec_rp_card(
        &mut self,
        calc_mode: ::std::os::raw::c_int,
        n_theta: ::std::os::raw::c_int,
        n_phi: ::std::os::raw::c_int,
        output_format: ::std::os::raw::c_int,
        normalization: ::std::os::raw::c_int,
        D: ::std::os::raw::c_int,
        A: ::std::os::raw::c_int,
        theta0: f64,
        phi0: f64,
        delta_theta: f64,
        delta_phi: f64,
        radial_distance: f64,
        gain_norm: f64,
    ) {
        eprintln!(
            "RP {} {} {} {} {} {} {} {} {} {} {} {} {}",
            calc_mode,
            n_theta,
            n_phi,
            output_format,
            normalization,
            D,
            A,
            theta0,
            phi0,
            delta_theta,
            delta_phi,
            radial_distance,
            gain_norm
        );
        check_result(unsafe {
            bindings::nec_rp_card(
                self.ptr,
                calc_mode,
                n_theta,
                n_phi,
                output_format,
                normalization,
                D,
                A,
                theta0,
                phi0,
                delta_theta,
                delta_phi,
                radial_distance,
                gain_norm,
            )
        })
    }

    pub fn nec_pt_card(
        &mut self,
        itmp1: ::std::os::raw::c_int,
        itmp2: ::std::os::raw::c_int,
        itmp3: ::std::os::raw::c_int,
        itmp4: ::std::os::raw::c_int,
    ) {
        check_result(unsafe { bindings::nec_pt_card(self.ptr, itmp1, itmp2, itmp3, itmp4) })
    }

    pub fn nec_pq_card(
        &mut self,
        itmp1: ::std::os::raw::c_int,
        itmp2: ::std::os::raw::c_int,
        itmp3: ::std::os::raw::c_int,
        itmp4: ::std::os::raw::c_int,
    ) {
        check_result(unsafe { bindings::nec_pq_card(self.ptr, itmp1, itmp2, itmp3, itmp4) })
    }

    pub fn nec_kh_card(&mut self, tmp1: f64) {
        check_result(unsafe { bindings::nec_kh_card(self.ptr, tmp1) })
    }

    pub fn nec_ne_card(
        &mut self,
        itmp1: ::std::os::raw::c_int,
        itmp2: ::std::os::raw::c_int,
        itmp3: ::std::os::raw::c_int,
        itmp4: ::std::os::raw::c_int,
        tmp1: f64,
        tmp2: f64,
        tmp3: f64,
        tmp4: f64,
        tmp5: f64,
        tmp6: f64,
    ) {
        check_result(unsafe {
            bindings::nec_ne_card(
                self.ptr, itmp1, itmp2, itmp3, itmp4, tmp1, tmp2, tmp3, tmp4, tmp5, tmp6,
            )
        })
    }

    pub fn nec_nh_card(
        &mut self,
        itmp1: ::std::os::raw::c_int,
        itmp2: ::std::os::raw::c_int,
        itmp3: ::std::os::raw::c_int,
        itmp4: ::std::os::raw::c_int,
        tmp1: f64,
        tmp2: f64,
        tmp3: f64,
        tmp4: f64,
        tmp5: f64,
        tmp6: f64,
    ) {
        check_result(unsafe {
            bindings::nec_nh_card(
                self.ptr, itmp1, itmp2, itmp3, itmp4, tmp1, tmp2, tmp3, tmp4, tmp5, tmp6,
            )
        })
    }

    pub fn nec_cp_card(
        &mut self,
        itmp1: ::std::os::raw::c_int,
        itmp2: ::std::os::raw::c_int,
        itmp3: ::std::os::raw::c_int,
        itmp4: ::std::os::raw::c_int,
    ) {
        check_result(unsafe { bindings::nec_cp_card(self.ptr, itmp1, itmp2, itmp3, itmp4) })
    }

    pub fn nec_pl_card(
        &mut self,
        //ploutput_filename: *mut ::std::os::raw::c_char,
        ploutput_filename: &Path,
        itmp1: ::std::os::raw::c_int,
        itmp2: ::std::os::raw::c_int,
        itmp3: ::std::os::raw::c_int,
        itmp4: ::std::os::raw::c_int,
    ) {
        let outfile = CString::new(ploutput_filename.to_str().unwrap()).unwrap();
        check_result(unsafe {
            bindings::nec_pl_card(
                self.ptr,
                outfile.as_ptr() as *const i8,
                itmp1,
                itmp2,
                itmp3,
                itmp4,
            )
        })
    }

    pub fn nec_gain(
        &mut self,
        freq_index: ::std::os::raw::c_int,
        theta_index: ::std::os::raw::c_int,
        phi_index: ::std::os::raw::c_int,
    ) -> f64 {
        unsafe { bindings::nec_gain(self.ptr, freq_index, theta_index, phi_index) }
    }

    pub fn nec_gain_max(&mut self, freq_index: ::std::os::raw::c_int) -> f64 {
        unsafe { bindings::nec_gain_max(self.ptr, freq_index) }
    }

    pub fn nec_gain_min(&mut self, freq_index: ::std::os::raw::c_int) -> f64 {
        unsafe { bindings::nec_gain_min(self.ptr, freq_index) }
    }

    pub fn nec_gain_mean(&mut self, freq_index: ::std::os::raw::c_int) -> f64 {
        unsafe { bindings::nec_gain_mean(self.ptr, freq_index) }
    }

    pub fn nec_gain_sd(&mut self, freq_index: ::std::os::raw::c_int) -> f64 {
        unsafe { bindings::nec_gain_sd(self.ptr, freq_index) }
    }

    pub fn nec_gain_rhcp_max(&mut self, freq_index: ::std::os::raw::c_int) -> f64 {
        unsafe { bindings::nec_gain_rhcp_max(self.ptr, freq_index) }
    }

    pub fn nec_gain_rhcp_min(&mut self, freq_index: ::std::os::raw::c_int) -> f64 {
        unsafe { bindings::nec_gain_rhcp_min(self.ptr, freq_index) }
    }

    pub fn nec_gain_rhcp_mean(&mut self, freq_index: ::std::os::raw::c_int) -> f64 {
        unsafe { bindings::nec_gain_rhcp_mean(self.ptr, freq_index) }
    }

    pub fn nec_gain_rhcp_sd(&mut self, freq_index: ::std::os::raw::c_int) -> f64 {
        unsafe { bindings::nec_gain_rhcp_sd(self.ptr, freq_index) }
    }

    pub fn nec_gain_lhcp_max(&mut self, freq_index: ::std::os::raw::c_int) -> f64 {
        unsafe { bindings::nec_gain_lhcp_max(self.ptr, freq_index) }
    }

    pub fn nec_gain_lhcp_min(&mut self, freq_index: ::std::os::raw::c_int) -> f64 {
        unsafe { bindings::nec_gain_lhcp_min(self.ptr, freq_index) }
    }

    pub fn nec_gain_lhcp_mean(&mut self, freq_index: ::std::os::raw::c_int) -> f64 {
        unsafe { bindings::nec_gain_lhcp_mean(self.ptr, freq_index) }
    }

    pub fn nec_gain_lhcp_sd(&mut self, freq_index: ::std::os::raw::c_int) -> f64 {
        unsafe { bindings::nec_gain_lhcp_sd(self.ptr, freq_index) }
    }

    pub fn nec_impedance_real(&mut self, freq_index: ::std::os::raw::c_int) -> f64 {
        unsafe { bindings::nec_impedance_real(self.ptr, freq_index) }
    }

    pub fn nec_impedance_imag(&mut self, freq_index: ::std::os::raw::c_int) -> f64 {
        unsafe { bindings::nec_impedance_imag(self.ptr, freq_index) }
    }
}
