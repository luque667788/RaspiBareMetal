// Utility functions for manipulating bits in registers
// TODO! consider using inline assembly for performance-critical operations


// function that allows you to set a certain bit in a register (u32)
pub fn set_bit(register: &mut u32, bit: u8) {
    *register |= 1 << bit;
}
// function that allows you to clear a certain bit in a register (u32)
pub fn clear_bit(register: &mut u32, bit: u8) {
    *register &= !(1 << bit);
}
// function that allows you to toggle a certain bit in a register (u32)
pub fn toggle_bit(register: &mut u32, bit: u8) {
    *register ^= 1 << bit;
}
// function that allows you to check if a certain bit is set in a register (u32)
pub fn is_bit_set(register: u32, bit: u8) -> bool {
    (register & (1 << bit)) != 0
}
// function that allows you to read a certain bit in a register (u32)
pub fn read_bit(register: u32, bit: u8) -> u8 {
    ((register >> bit) & 1) as u8
}