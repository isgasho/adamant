static GDT_ENTRY_COUNT: u8 = 5;
static GDT_LONG_MODE_GRANULARITY: u8 = 2;
static GDT_SEGMENT: u32 = 0b10000;
static GDT_PRESENT: u32 = 0b10000000;
static GDT_TSS_PRESENT: u32 = 0b10000000;
static GDT_USER: u8 = 0b01100000;
static GDT_EXECUTABLE: u8 = 0b00001000;
static GDT_READWRITE: u8 = 0b00000010;
static GDT_FLAGS: u8 = 0b1100;
static TSS_FLAGS: u8 = 0b0000;

#[derive(Default, Copy, Clone)]
#[repr(packed)]
pub struct TSS64 {
    reserved: u32,

    rsp: [u64; 3],
    
    reserved0: u64,
    ist: [u64; 7],
    
    reserved1: u32,
    reserved2: u32,
    reserved3: u32,
    
    iopb_offset: u16,
}

impl TSS64 {
    pub fn with_stack(stack: *const u8) -> Self {
        let mut to_return = TSS64::default();
        to_return.rsp[0] = stack as u64;
        to_return.ist[0] = stack as u64;
        to_return.clone()
    }
}

#[repr(packed)]
pub struct GDTDescriptor64 {
    size: u16,
    offset: u64,
}

impl GDTDescriptor64 {
    pub fn new(size: u16, offset: u64) -> Self {
        Self {
            size,
            offset,
        }
    }
}

#[derive(Default)]
pub struct LimitPlusFlags(u8);

impl LimitPlusFlags {
    pub fn new(limit16_19: u8, flags: u8) -> Self {
        Self(0b11111111)
    }

    pub fn limit16_19(&self) -> u8 {
        self.0 & 0xf
    }

    pub fn flags(&self) -> u8 {
        (self.0 & 0xf0) >> 4
    }
}

#[repr(packed)]
#[derive(Default)]
pub struct GDTEntry64 {
    limit0_15: u16,
    base0_15: u16,
    base16_23: u8,
    access_byte: u8,
    limit16_19_and_flags: LimitPlusFlags,  // First part of the u8 is the limit16_19, 2nd one is the granularity
    base24_31: u8,
}

impl GDTEntry64 {
    pub fn new(base: u32, limit: u32, flags: u8, access_byte: u8) -> Self {
        Self {
            limit0_15: (limit as u16) & 0xffff,
            base0_15: (base as u16) & 0xffff,
            base16_23: ((base as u8) >> 16) & 0xff,
            access_byte: flags,
            limit16_19_and_flags: LimitPlusFlags::new(((limit >> 16) & 0x0f) as u8, flags),
            base24_31: ((base as u8) >> 24) & 0xff,
        }
    }

    pub fn tss_entry(tss: *const TSS64, access: u8, flags: u8) -> Self {
        let tss_size = core::mem::size_of::<TSS64>() as u32;
        Self::new(tss as u32, tss as u32 + tss_size, access, flags)
    }
}

#[repr(packed)]
#[derive(Default)]
pub struct GdtTssEntry64 {
    length: u16,

    base_low16: u16,
    base_mid8: u8,
    
    flags1: u8,
    flags2: u8,
    
    base_high8: u8,
    base_upper32: u32,
    
    reserved: u32,
}

impl GdtTssEntry64 {
    pub fn tss_entry(tss: u64) -> Self {
        Self {
            length: core::mem::size_of::<TSS64>() as u16,
            base_low16: (tss & 0xffff) as u16,
            base_mid8: ((tss >> 16) & 0xff) as u8,
            flags1: 0b10001001,
            flags2: 0,
            base_high8: ((tss >> 24) & 0xff) as u8,
            base_upper32: (tss >> 32) as u32,
            reserved: 0,
        }
    }
}

#[repr(packed)]
#[derive(Default)]
pub struct GDT64 {
    entries: [GDTEntry64; 5],
    tss: GdtTssEntry64,
}

impl GDT64 {
    pub fn add_entry(&mut self, entry_index: usize, entry: GDTEntry64) {
        self.entries[entry_index] = entry;
    }

    pub fn set_tss_entry(&mut self, tss_entry: GdtTssEntry64) {
        self.tss = tss_entry;
    }
}

lazy_static! {
    static ref TSS: TSS64 = TSS64::default();

    static ref GDT: GDT64 = {
        let mut gdt = GDT64::default();

        gdt.add_entry(0, 
            GDTEntry64::new(0, 0, 0, 0));
        
        gdt.add_entry(1, 
            GDTEntry64::new(GDT_PRESENT, GDT_SEGMENT, GDT_READWRITE, GDT_EXECUTABLE));

        gdt.add_entry(2, 
            GDTEntry64::new(GDT_PRESENT, GDT_SEGMENT, GDT_READWRITE, 0));

        gdt.add_entry(3, 
            GDTEntry64::new(GDT_PRESENT, GDT_SEGMENT, GDT_READWRITE, GDT_EXECUTABLE));
        
        gdt.add_entry(4,
            GDTEntry64::new(GDT_PRESENT, GDT_SEGMENT, GDT_READWRITE, GDT_USER));
        
        gdt.set_tss_entry(GdtTssEntry64::tss_entry( (&TSS as *const TSS) as u64 ));

        gdt
    };

    static ref GDT_DESCRIPTOR: GDTDescriptor64 = GDTDescriptor64::new(core::mem::size_of::<GDT64>() as u16, (&GDT as *const GDT) as u64);
}

use crate::{print, println};
pub fn load_gdt() {
    unsafe {
        println!("Loading GDT...");
        super::loader::_x86_64_asm_lgdt(&*GDT_DESCRIPTOR);
        println!("GDT Loaded!")
    }
}