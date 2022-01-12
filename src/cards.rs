use crate::raw_nec_context::RawNecContext;

pub trait Card {
    fn add_to(&self, context: &mut RawNecContext);
}

//#[derive(Clone, Copy)]
pub enum SurfacePatch {
    Arbitrary {
        xc: f64,
        yc: f64,
        zc: f64,
        el: f64,
        az: f64,
        area: f64,
    },
    Rectangular {
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
        x3: f64,
        y3: f64,
        z3: f64,
    },
    Triangular {
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
        x3: f64,
        y3: f64,
        z3: f64,
    },
    Quadrilateral {
        x1: f64,
        y1: f64,
        z1: f64,
        x2: f64,
        y2: f64,
        z2: f64,
        x3: f64,
        y3: f64,
        z3: f64,
        x4: f64,
        y4: f64,
        z4: f64,
    },
}

impl Card for SurfacePatch {
    fn add_to(&self, context: &mut RawNecContext) {
        match self {
            &Self::Arbitrary {
                xc,
                yc,
                zc,
                el,
                az,
                area,
            } => context.nec_sp_card(0, xc, yc, zc, el, az, area),
            &Self::Rectangular {
                x1,
                y1,
                z1,
                x2,
                y2,
                z2,
                x3,
                y3,
                z3,
            } => {
                context.nec_sp_card(1, x1, y1, z1, x2, y2, z2);
                context.nec_sc_card(1, x3, y3, z3, 0.0, 0.0, 0.0);
            }
            &Self::Triangular {
                x1,
                y1,
                z1,
                x2,
                y2,
                z2,
                x3,
                y3,
                z3,
            } => {
                context.nec_sp_card(2, x1, y1, z1, x2, y2, z2);
                context.nec_sc_card(2, x3, y3, z3, 0.0, 0.0, 0.0);
            }
            &Self::Quadrilateral {
                x1,
                y1,
                z1,
                x2,
                y2,
                z2,
                x3,
                y3,
                z3,
                x4,
                y4,
                z4,
            } => {
                context.nec_sp_card(3, x1, y1, z1, x2, y2, z2);
                context.nec_sc_card(3, x3, y3, z3, x4, y4, z4);
            }
        }
    }
}

pub struct Gm {
    pub itsi: i32,
    pub nrpt: i32,
    pub rox: f64,
    pub roy: f64,
    pub roz: f64,
    pub xs: f64,
    pub ys: f64,
    pub zs: f64,
    pub its: f64,
}

impl Card for Gm {
    fn add_to(&self, context: &mut RawNecContext) {
        context.nec_gm_card(
            self.itsi,
            self.nrpt,
            self.rox,
            self.roy,
            self.roz,
            self.xs,
            self.ys,
            self.zs,
            self.its as i32,
        );
    }
}

pub struct Gx {
    pub i1: i32,
    pub i2: i32,
}

impl Card for Gx {
    fn add_to(&self, context: &mut RawNecContext) {
        context.nec_gx_card(self.i1, self.i2)
    }
}

pub struct Ge {
    pub gpflag: i32,
}

impl Card for Ge {
    fn add_to(&self, context: &mut RawNecContext) {
        context.nec_geometry_complete(self.gpflag)
    }
}

pub enum Gn {
    Null,
    Finite {
        nradl: i32,
        epse: f64,
        sig: f64,
        screen_rad: f64,
        wire_rad: f64,
    },
    PerfectlyConducting,
    SecondMedium {
        epse1: f64,
        sig1: f64,
        epse2: f64,
        sig2: f64,
        r_join: f64,
        depth_join: f64,
    },
}

impl Card for Gn {
    fn add_to(&self, context: &mut RawNecContext) {
        match self {
            &Self::Null => context.nec_gn_card(-1, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            &Self::Finite {
                nradl,
                epse,
                sig,
                screen_rad,
                wire_rad,
            } => context.nec_gn_card(0, nradl, epse, sig, screen_rad, wire_rad, 0.0, 0.0),
            &Self::PerfectlyConducting => context.nec_gn_card(1, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            &Self::SecondMedium {
                epse1,
                sig1,
                epse2,
                sig2,
                r_join,
                depth_join,
            } => context.nec_gn_card(1, 0, epse1, sig1, epse2, sig2, r_join, depth_join),
        }
    }
}

enum Fr {
    Linear {
        nfreq: i32,
        f0_MHz: f64,
        df_MHz: f64,
    },
    Mul {
        nfreq: i32,
        f0_MHz: f64,
        ffactor: f64,
    },
}

impl Card for Fr {
    fn add_to(&self, context: &mut RawNecContext) {
        match self {
            &Self::Linear {
                nfreq,
                f0_MHz,
                df_MHz,
            } => context.nec_fr_card(0, nfreq, f0_MHz, df_MHz),
            &Self::Mul {
                nfreq,
                f0_MHz,
                ffactor,
            } => context.nec_fr_card(1, nfreq, f0_MHz, ffactor),
        }
    }
}

enum Ek {
    Standard,
    Thin,
}

impl Card for Ek {
    fn add_to(&self, context: &mut RawNecContext) {
        match self {
            &Ek::Standard => context.nec_ek_card(-1),
            &Ek::Thin => context.nec_ek_card(0),
        }
    }
}

//=============================================================
pub struct Wire {
    pub tag: i32,
    pub ns: i32,
    pub x1: f64,
    pub y1: f64,
    pub z1: f64,
    pub rad: f64,
    pub rdel: f64,
    pub rrad: f64,
}
