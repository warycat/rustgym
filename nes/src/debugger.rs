use crate::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Default)]
pub struct Debugger {
    code_data_logger: CodeDataLogger,
    code_runner: CodeRunner,
    dummy_cpu: DummyCpu,
    bp_dummy_cpu_required: bool,
    break_on_first_cycle: bool,
    has_script: bool,
    next_script_id: i32,
    rpn_list: [Vec<ExpressionData>; BreakpointType::COUNT],
    has_breakpoint: bool,
    frozen_addresses: Vec<u8>,
    last_instruction: u8,
    op_code_cycle: u32,
    memory_operation: MemoryOperation,
    sub_return_addresses: VecDeque<i32>,
    step_out_return_address: i32,
    watch_exp_eval: ExpressionEvaluator,
    bp_exp_eval: ExpressionEvaluator,
    current_read_addr: u16,
    current_read_value: u8,
    next_read_addr: u16,
    return_to_address: u16,
    rom_name: String,
    break_source: BreakSource,
    enable_break_on_uninit_read: bool,
    paused_for_debug_healper: bool,
    process_ppu_cycle: Vec<bool>,
    ppu_viewer_update_cycle: HashMap<i32, i32>,
    ppu_scroll_x: u16,
    ppu_scroll_y: u16,
    prev_instruction_cycle: i64,
    cur_instruction_cycle: i64,
    run_to_cycle: i64,
    need_rewind: bool,
    rewind_cache: Vec<Box<dyn std::io::Write>>,
    rewind_pre_instruction_cycle_cache: Vec<u64>,
    pub callstack: VecDeque<StackFrameInfo>,
    pub flags: u32,
    pub breakpoints: [Vec<Breakpoint>; BreakpointType::COUNT],
    pub label_manager: LabelManager,
    pub function_entry_points: HashSet<u32>,
    pub debug_state: DebugState,
    pub profiler: Profiler,
    pub disassembler: Disassembler,
    pub trace_logger: TraceLogger,
    pub memory_dumper: MemoryDumper,
    pub memory_access_counter: MemoryAccessCounter,
    pub performace_tracker: PerformanceTracker,
    pub event_manager: EventManager,
}

impl Debugger {
    fn process_breakpoints(
        &mut self,
        breakpoint_type: BreakpointType,
        operation_info: OperationInfo,
        allow_break: bool,
        allow_mark: bool,
    ) -> bool {
        todo!()
    }

    fn process_all_break_breakpoints(&mut self, operation_info: &OperationInfo) {
        todo!()
    }

    fn add_callstack_frame(&mut self, source: u16, target: u16, flags: StackFrameFlags) {
        todo!()
    }

    fn update_callstack(&mut self, current_instruction: u8, addr: u32) {
        todo!()
    }

    fn process_step_conditions(&mut self, addr: u16) {
        todo!()
    }

    fn sleep_until_resume(
        &mut self,
        source: BreakSource,
        breakpoint_id: u32,
        breakpoint_type: BreakpointType,
        breakpoint_address: u16,
        breakpoint_value: u8,
        memory_operation: MemoryOperation,
    ) {
        todo!()
    }

    fn update_ppu_cycles_to_process(&mut self) {
        todo!()
    }

    fn reset_step_state(&mut self) {
        todo!()
    }

    pub fn get_instruction_progress(&self) -> InstructionProgress {
        todo!()
    }

    pub fn suspend(&mut self) {
        todo!()
    }

    pub fn resume(&mut self) {
        todo!()
    }

    pub fn break_(&mut self) {
        todo!()
    }

    pub fn resume_from_break(&mut self) {
        todo!()
    }

    pub fn ppu_step(&mut self, count: u32) {
        todo!()
    }

    pub fn step(&mut self, count: u32, source: BreakSource) {
        todo!()
    }

    pub fn step_cycles(&mut self, cycle_count: u32) {
        todo!()
    }

    pub fn step_over(&mut self) {
        todo!()
    }

    pub fn step_out(&mut self) {
        todo!()
    }

    pub fn run(&mut self) {
        todo!()
    }

    pub fn break_immediately(&mut self, source: BreakSource) {
        todo!()
    }

    pub fn break_on_scanline(&mut self, scanline: u16) {
        todo!()
    }

    pub fn is_mark_as_code(&self, relative_address: u16) -> bool {
        todo!()
    }

    pub fn set_next_statement(&mut self, addr: u16) {
        todo!()
    }

    pub fn set_ppu_viewer_scanline_cycle(&mut self, ppu_viewer_id: i32, scanline: i32, cycle: i32) {
        todo!()
    }

    pub fn clear_ppu_viewer_settings(&mut self, ppu_viewer: i32) {
        todo!()
    }

    pub fn is_execution_stopped(&self) -> bool {
        todo!()
    }

    pub fn is_pause_icon_shown(&self) -> bool {
        todo!()
    }

    pub fn prevent_resume(&mut self) {
        todo!()
    }

    pub fn allow_resume(&mut self) {
        todo!()
    }

    pub fn generate_code_output(&mut self) {
        todo!()
    }

    pub fn get_code(&self) -> String {
        todo!()
    }

    pub fn get_relative_cpu_address(addr: u32, cpu_address_type: CpuAddressType) -> i32 {
        todo!()
    }

    pub fn get_relative_ppu_address(addr: u32, ppu_address_type: PpuAddressType) -> i32 {
        todo!()
    }

    pub fn get_absolute_address(addr: u32) -> i32 {
        todo!()
    }

    pub fn get_absolute_chr_address(addr: u32) -> i32 {
        todo!()
    }

    pub fn get_absolute_cpu_address_info(relative_addr: u32) -> CpuAddressInfo {
        todo!()
    }

    pub fn get_absolute_ppu_address_info(relative_addr: u32) -> PpuAddressInfo {
        todo!()
    }

    pub fn evaluate_expression(
        expression: String,
        eval_result_type: EvalResultType,
        use_cache: bool,
    ) -> i32 {
        todo!()
    }

    pub fn is_ppu_cycle_to_process(&self) -> bool {
        todo!()
    }

    pub fn process_ppu_cycle(&mut self) {
        todo!()
    }

    pub fn process_ram_operation(
        &mut self,
        memory_operation: MemoryOperation,
        addr: u16,
        value: u8,
    ) {
        todo!()
    }

    pub fn process_vram_operation(
        &mut self,
        memory_operation: MemoryOperation,
        addr: u16,
        value: u8,
    ) {
        todo!()
    }

    pub fn set_last_frame_ppu_scool(addr: u16, x_scroll: u8, update_horizontal_scroll_only: bool) {
        todo!()
    }

    pub fn process_interrupt(&mut self, cpu_addr: u16, dest_cpu_addr: u16, for_nmi: bool) {
        todo!()
    }

    pub fn add_trace(&mut self, log: &str) {
        todo!()
    }

    pub fn set_free_state(&mut self, address: u32, frozen: bool) {
        todo!()
    }

    pub fn get_freeze_state(&self, length: u16) -> bool {
        todo!()
    }

    pub fn start_code_runner(&mut self, byte_code: &[u8]) {
        todo!()
    }

    pub fn stop_code_runner(&mut self) {
        todo!()
    }

    pub fn revert_prg_chr_changes(&mut self) {
        todo!()
    }

    pub fn has_prg_chr_changes(&self) -> bool {
        todo!()
    }

    pub fn find_sub_entry_point(relative_address: u16) -> i32 {
        todo!()
    }

    pub fn set_input_override(&mut self, port: u8, state: u32) {
        todo!()
    }

    pub fn load_script(&mut self, name: String, content: String, script_id: i32) -> i32 {
        todo!()
    }

    pub fn remove_script(&mut self, script_id: i32) {
        todo!()
    }

    pub fn get_script_log(&self, script_id: i32) -> &str {
        todo!()
    }

    pub fn reset_counters(&mut self) {
        todo!()
    }

    pub fn process_script_save_state(&mut self, addr: u16, value: u8) {
        todo!()
    }

    pub fn process_cpu_operation(
        &mut self,
        addr: u16,
        value: u8,
        memory_operation: MemoryOperation,
    ) {
        todo!()
    }

    pub fn process_ppu_operation(
        &mut self,
        addr: u16,
        value: u8,
        memory_operation: MemoryOperation,
    ) {
        todo!()
    }

    pub fn process_event(&mut self, event: Event) {
        todo!()
    }

    pub fn get_screen_pixel(x: u8, y: u8) -> u32 {
        todo!()
    }

    pub fn set_last_frame_ppu_scroll(
        &mut self,
        addr: u16,
        x_scroll: u8,
        update_horizontal_scroll_only: bool,
    ) {
        self.ppu_scroll_x =
            (addr & 0x1F) << 3 | x_scroll as u16 | if addr & 0x400 == 0x400 { 0x100 } else { 0 };
        if update_horizontal_scroll_only {
            self.ppu_scroll_y = ((addr & 0x3E0) >> 2 | (addr & 0x7000) >> 12)
                + if addr & 0x800 == 0x800 { 240 } else { 0 };
        }
    }

    pub fn add_debug_event(&mut self, debug_event: DebugEvent) {
        todo!()
    }
}
