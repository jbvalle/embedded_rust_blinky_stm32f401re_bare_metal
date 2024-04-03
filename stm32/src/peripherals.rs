#[repr(C)]
pub struct GPIOx {
    pub moder: u32,
    pub otyper: u32,
    pub ospeedr: u32,
    pub pupdr: u32,
    pub idr: u32,
    pub odr: u32,
    pub bsrr: u32,
    pub lckr: u32,
    pub afrl: u32,
    pub afrh: u32,
}


#[repr(C)]
pub struct RCC {
    pub cr: u32,
    pub pllcfgr: u32,
    pub cfgr: u32,
    pub cir: u32,
    pub ahb1rstr: u32,
    pub ahb2rstr: u32,
    pub _res1: [u32; 2],
    pub apb1rstr: u32,
    pub apb2rstr: u32,
    pub _res2: [u32; 2],
    pub ahb1enr: u32,
    pub ahb2enr: u32,
    pub _res3: [u32; 2],
    pub apb1enr: u32,
    pub apb2enr: u32,
    pub _res4: [u32; 2],
    pub ahb1lpenr: u32,
    pub ahb2lpenr: u32,
    pub _res5: [u32; 2],
    pub apb1lpenr: u32,
    pub apb2lpenr: u32,
    pub _res6: [u32; 2],
    pub bdcr: u32,
    pub csr: u32,
    pub _res7: [u32; 2],
    pub sscgr: u32,
    pub plli2scfgr: u32,
    pub _res8: u32,
    pub dckcfgr: u32,
}

