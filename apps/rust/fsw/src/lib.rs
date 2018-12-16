#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::thread;
use std::ffi::CString;
use std::mem;

use std::os::raw::c_void;

extern crate cfs_sys;
use cfs_sys::*;
//use cfs_sys::raw::*;


// performance ID for this App's main task
const RUST_APPMAIN_PERF_ID : uint32 = 0x8888;

// Time to wait for system to start
const RUST_STARTUP_SYNC_TIMEOUT : uint32 = 65000;

const RUST_HK_TLM_MID : uint16 = 0x18fA;

const RUST_CMD_PIPE_DEPTH : uint16 = 10;

const RUST_CMD_PIPE_NAME : &str = "RustTlm";

const RUST_SEND_HK_MID : uint16 = 0x18f9;

const RUST_APP_INF_EID : u16 = 0x8888;


// The App's global data. This is a convention in
// CFS code where all data related to an App is contained
// in a single struct.
pub struct RustAppData {
    run_status : uint32,

    app_id : uint32,

    tlm_packet : RustTlm,

    // pipe used to receive command packets.
    cmd_pipe : CFE_SB_PipeId_t,
}

// The App's housekeeping packet. This is defined with
// repr(C) to be consistent with structs in the main CFS code.
//
// The TlmHeader ensures that there is space for a CCSDS Primary
// Header, as this struct describes the entire packet, including
// the header.
#[repr(C)]
struct RustTlm {
    pub TlmHeader: [uint8; 12usize],

    pub cmd_received_counter : uint16,
    pub tlm_send_counter     : uint16,

    pub tlm_err_counter  : uint8,
    pub cmd_err_counter  : uint8,
}

impl Default for RustTlm {
    fn default() -> Self {
        RustTlm {
          TlmHeader            : [0; 12usize],
          cmd_received_counter : 0,
          tlm_send_counter     : 0,
          tlm_err_counter      : 0,
          cmd_err_counter      : 0,
        }
    }
}

// wrapper around per log add, which is defined as a 
// macro in the C code.
fn CFE_ES_PerfLogEntry(perf_id : uint32) {
    unsafe {
        CFE_ES_PerfLogAdd(perf_id, 0);
    }
}

// wrapper around per log exit, which is defined as a 
// macro in the C code.
fn CFE_ES_PerfLogExit(perf_id : uint32) {
    unsafe {
        CFE_ES_PerfLogAdd(perf_id, 1);
    }
}

/// This function is the main entry point into this module.
/// When the object file is loaded by cFE at startup, this is
/// the function is will call. This requires configuring 
/// the cfe_es_startup.scr file to load the Rust App.
#[no_mangle]
pub extern fn Rust_AppInit() {
    let mut status = 0;

    let mut run_status : uint32 = CFE_ES_RUNSTATUS_APP_RUN;
    let mut app_id = 0;
    let mut tlm_packet : RustTlm = Default::default();
    let mut cmd_pipe : CFE_SB_PipeId_t = 0;

    // we need a pointer to a pointer to receive a buffer from the
    // SB module.
    let mut rcv_packet : *mut CFE_SB_MsgPtr_t = 0 as *mut CFE_SB_MsgPtr_t;

    // This whole thing is wrapped in unsafe, as all calls to the C
    // bindings are unsafe.
    unsafe {
        // log initialization to perf monitoring
        CFE_ES_PerfLogEntry(RUST_APPMAIN_PERF_ID);

        // register this application with ES
        status = CFE_ES_RegisterApp();

        // Get current Apps ID
        CFE_ES_GetAppID(&mut app_id);

        // Log a startup message to the system log
        CFE_ES_WriteToSysLog(CString::new("Rust App running as %d!\n").unwrap().as_ptr(), app_id);

        status = CFE_EVS_Register(0 as *mut c_void,
                                  0,
                                  CFE_EVS_BINARY_FILTER as u16);
        if status != CFE_SUCCESS as i32 {
            // write out to the system log
            CFE_ES_WriteToSysLog(CString::new("RUST (%d) Registering with EVS failed!\n").unwrap().as_ptr(), app_id);
        }

        // set up a telemetry packet
        CFE_SB_InitMsg(mem::transmute::<&mut RustTlm, *mut c_void>(mem::transmute(&mut tlm_packet)),
                       RUST_HK_TLM_MID,
                       mem::size_of::<RustTlm>() as uint16,
                       TRUE as u8);

        // return should be checked before cast to u8
        status = CFE_SB_CreatePipe(mem::transmute(&mut cmd_pipe),
                                   RUST_CMD_PIPE_DEPTH,
                                   RUST_CMD_PIPE_NAME.as_ptr() as *const i8);
        status = CFE_SB_Subscribe(RUST_SEND_HK_MID, cmd_pipe);

        // Don't start main loop until the rest of the system is ready!
        CFE_ES_WaitForStartupSync(RUST_STARTUP_SYNC_TIMEOUT);
    }

    while run_status == CFE_ES_RUNSTATUS_APP_RUN {
        unsafe {
            CFE_ES_PerfLogExit(RUST_APPMAIN_PERF_ID);

            // wait for a command, or a message asking for a housekeeping packet
            status = CFE_SB_RcvMsg(mem::transmute(&mut rcv_packet),
                                   cmd_pipe,
                                   CFE_SB_PEND_FOREVER as i32);

            if status == CFE_SUCCESS as i32 {
                tlm_packet.cmd_received_counter += 1;

                let cmd_packet : &CFE_SB_CmdHdr_t = mem::transmute(*rcv_packet);

                // send telemetry packet off to SB
                CFE_SB_InitMsg(mem::transmute::<&mut RustTlm, *mut c_void>(mem::transmute(&mut tlm_packet)),
                               RUST_HK_TLM_MID,
                               mem::size_of::<RustTlm>() as uint16,
                               TRUE as u8);
                CFE_SB_TimeStampMsg(mem::transmute(&mut tlm_packet));
                CFE_SB_SendMsg(mem::transmute(&mut tlm_packet));

                CFE_EVS_SendEvent(RUST_APP_INF_EID, CFE_EVS_INFORMATION as u16, "Rust telemetry sent".as_ptr() as *const i8);
            }
            else
            {
                CFE_EVS_SendEvent(RUST_APP_INF_EID, CFE_EVS_INFORMATION as u16, "Rust Command Error %d 0x%08X ".as_ptr() as *const i8, status, status);
                OS_TaskDelay(1000);
            }

            run_status = CFE_ES_RunLoop(&mut run_status) as uint32;
            CFE_ES_PerfLogEntry(RUST_APPMAIN_PERF_ID);
        }
    }

    unsafe {
        CFE_ES_PerfLogExit(RUST_APPMAIN_PERF_ID);

        // report app exit before exiting
        CFE_ES_ExitApp(run_status);
    }
}
