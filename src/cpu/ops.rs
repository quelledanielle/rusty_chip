use cpu::opcode::Opcode;

pub trait Operation {
    fn no_op(&mut self, &Opcode);
    fn unknown(&mut self, &Opcode);
    fn clear_display(&mut self, &Opcode);
    fn return_from_subroutine(&mut self, &Opcode);
    fn jump_addr(&mut self, &Opcode);
    fn call_addr(&mut self, &Opcode);
    fn skip_equal_vx_byte(&mut self, &Opcode);
    fn skip_not_equal_vx_byte(&mut self, &Opcode);
    fn skip_equal_vx_vy(&mut self, &Opcode);
    fn load_vx_byte(&mut self, &Opcode);
    fn add_vx_byte(&mut self, &Opcode);
    fn load_vx_vy(&mut self, &Opcode);
    fn or_vx_vy(&mut self, &Opcode);
    fn and_vx_vy(&mut self, &Opcode);
    fn xor_vx_vy(&mut self, &Opcode);
    fn add_vx_vy(&mut self, &Opcode);
    fn sub_vx_vy(&mut self, &Opcode);
    fn shr_vx_vy(&mut self, &Opcode);
    fn subn_vx_vy(&mut self, &Opcode);
    fn shl_vx_vy(&mut self, &Opcode);
    fn skip_not_equal_vx_vy(&mut self, &Opcode);
    fn load_i_addr(&mut self, &Opcode);
    fn jump_v0_addr(&mut self, &Opcode);
    fn rand_vx_byte(&mut self, &Opcode);
    fn draw_vx_vy_n(&mut self, &Opcode);
    fn skip_key_pressed_vx(&mut self, &Opcode);
    fn skip_key_not_pressed_vx(&mut self, &Opcode);
    fn load_vx_dt(&mut self, &Opcode);
    fn load_vx_key(&mut self, &Opcode);
    fn load_dt_vx(&mut self, &Opcode);
    fn load_st_vx(&mut self, &Opcode);
    fn add_i_vx(&mut self, &Opcode);
    fn load_i_vx_font(&mut self, &Opcode);
    fn load_bcd_vx(&mut self, &Opcode);
    fn load_through_vx(&mut self, &Opcode);
    fn read_through_vx(&mut self, &Opcode);
}
