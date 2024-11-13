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
        _breakpoint_type: BreakpointType,
        _operation_info: OperationInfo,
        _allow_break: bool,
        _allow_mark: bool,
    ) -> bool {
        todo!()
    }

    fn process_all_break_breakpoints(&mut self, _operation_info: &OperationInfo) {
        todo!()
    }

    fn add_callstack_frame(&mut self, _source: u16, _target: u16, _flags: StackFrameFlags) {
        todo!()
    }

    fn update_callstack(&mut self, _current_instruction: u8, _addr: u32) {
        todo!()
    }

    fn process_step_conditions(&mut self, _addr: u16) {
        todo!()
    }

    fn sleep_until_resume(
        &mut self,
        _source: BreakSource,
        _breakpoint_id: u32,
        _breakpoint_type: BreakpointType,
        _breakpoint_address: u16,
        _breakpoint_value: u8,
        _memory_operation: MemoryOperation,
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

    pub fn ppu_step(&mut self, _count: u32) {
        todo!()
    }

    pub fn step(&mut self, _count: u32, _source: BreakSource) {
        todo!()
    }

    pub fn step_cycles(&mut self, _cycle_count: u32) {
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

    pub fn break_immediately(&mut self, _source: BreakSource) {
        todo!()
    }

    pub fn break_on_scanline(&mut self, _scanline: u16) {
        todo!()
    }

    pub fn is_mark_as_code(&self, _relative_address: u16) -> bool {
        todo!()
    }

    pub fn set_next_statement(&mut self, _addr: u16) {
        todo!()
    }

    pub fn set_ppu_viewer_scanline_cycle(
        &mut self,
        _ppu_viewer_id: i32,
        _scanline: i32,
        _cycle: i32,
    ) {
        todo!()
    }

    pub fn clear_ppu_viewer_settings(&mut self, _ppu_viewer: i32) {
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

    pub fn get_relative_cpu_address(_addr: u32, _cpu_address_type: CpuAddressType) -> i32 {
        todo!()
    }

    pub fn get_relative_ppu_address(_addr: u32, _ppu_address_type: PpuAddressType) -> i32 {
        todo!()
    }

    pub fn get_absolute_address(_addr: u32) -> i32 {
        todo!()
    }

    pub fn get_absolute_chr_address(_addr: u32) -> i32 {
        todo!()
    }

    pub fn get_absolute_cpu_address_info(_relative_addr: u32) -> CpuAddressInfo {
        todo!()
    }

    pub fn get_absolute_ppu_address_info(_relative_addr: u32) -> PpuAddressInfo {
        todo!()
    }

    pub fn evaluate_expression(
        _expression: String,
        _eval_result_type: EvalResultType,
        _use_cache: bool,
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
        _memory_operation: MemoryOperation,
        _addr: u16,
        _value: u8,
    ) {
        todo!()
    }

    pub fn process_vram_operation(
        &mut self,
        _memory_operation: MemoryOperation,
        _addr: u16,
        _value: u8,
    ) {
        todo!()
    }

    pub fn set_last_frame_ppu_scool(
        _addr: u16,
        _x_scroll: u8,
        _update_horizontal_scroll_only: bool,
    ) {
        todo!()
    }

    pub fn process_interrupt(&mut self, _cpu_addr: u16, _dest_cpu_addr: u16, _for_nmi: bool) {
        todo!()
    }

    pub fn add_trace(&mut self, _log: &str) {
        todo!()
    }

    pub fn set_free_state(&mut self, _address: u32, _frozen: bool) {
        todo!()
    }

    pub fn get_freeze_state(&self, _length: u16) -> bool {
        todo!()
    }

    pub fn start_code_runner(&mut self, _byte_code: &[u8]) {
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

    pub fn find_sub_entry_point(_relative_address: u16) -> i32 {
        todo!()
    }

    pub fn set_input_override(&mut self, _port: u8, _state: u32) {
        todo!()
    }

    pub fn load_script(&mut self, _name: String, _content: String, _script_id: i32) -> i32 {
        todo!()
    }

    pub fn remove_script(&mut self, _script_id: i32) {
        todo!()
    }

    pub fn get_script_log(&self, _script_id: i32) -> &str {
        todo!()
    }

    pub fn reset_counters(&mut self) {
        todo!()
    }

    pub fn process_script_save_state(&mut self, _addr: u16, _value: u8) {
        todo!()
    }

    pub fn process_cpu_operation(
        &mut self,
        _addr: u16,
        _value: u8,
        _memory_operation: MemoryOperation,
    ) {
        todo!()
    }

    pub fn process_ppu_operation(
        &mut self,
        _addr: u16,
        _value: u8,
        _memory_operation: MemoryOperation,
    ) {
        todo!()
    }

    pub fn process_event(&mut self, _event: Event) {
        todo!()
    }

    pub fn get_screen_pixel(_x: u8, _y: u8) -> u32 {
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

    pub fn add_debug_event(&mut self, _debug_event: DebugEvent) {
        todo!()
    }
}
