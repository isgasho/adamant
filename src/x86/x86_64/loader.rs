use super::gdt::GDTDescriptor64;

extern "C" {
    pub fn _x86_64_asm_lgdt(gdt: *const GDTDescriptor64);

    //pub fn x86_64_asm_lidt(idt: *const crate::instructions::tables::DescriptorTablePointer);
}