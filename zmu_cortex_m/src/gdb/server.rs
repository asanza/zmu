//!
//! Flash Memory simulation
//!
//!

use gdbstub::stub::run_blocking;
use gdbstub::stub::SingleThreadStopReason;
use gdbstub::conn::Connection;
use gdbstub::conn::ConnectionExt;
use gdbstub::target::Target;
use gdbstub::common::Signal;
use gdbstub::stub::GdbStub;
use gdbstub::stub::DisconnectReason;

use crate::{MemoryMapConfig};
use crate::gdb::conn;
use conn::TcpConnection;
use crate::gdb::simulation::SimulationEvent;
use crate::gdb::simulation::SimulationRunEvent;

use crate::gdb::target::ZmuTarget;

use crate::semihosting::SemihostingCommand;
use crate::semihosting::SemihostingResponse;

///
/// The gdb Server
/// 
pub struct GdbServer {
    // number: i32,
    target: ZmuTarget,
}

/// 
impl GdbServer {

    ///
    pub fn new(
        code: &[u8],
        semihost_func: Box<dyn FnMut(&SemihostingCommand) -> SemihostingResponse + 'static>,
        map: Option<MemoryMapConfig>,
        flash_size: usize,
    ) -> Result<GdbServer, &'static str> {

        let target = ZmuTarget::new(code, semihost_func, map, flash_size);

        Ok(GdbServer {target})
    }

    ///
    pub fn start(&mut self) -> Result<u32, &'static str> {
        println!("Starting GDB server");
        let mut exit_code = 0;
        let conn = match conn::TcpConnection::new_localhost(9001) {
            Ok(conn) => conn,
            Err(e) => return Err(e),
        };

        let gdb = GdbStub::new(conn);

        match gdb.run_blocking::<EventLoop>(&mut self.target) {
            Ok(disconnect_reason) => match disconnect_reason {
                DisconnectReason::Disconnect => {
                    println!("GDB client has disconnected. Running to completion...");
                    loop {
                        match self.target.step() {
                            SimulationEvent::Halted => break,
                            SimulationEvent::Finalized(code) => {
                                exit_code = code;
                                break;
                            }
                            _ => {}
                        }
                    }
                }
                DisconnectReason::TargetExited(code) => {
                    println!("\nTarget exited with code {}!", code)
                }
                DisconnectReason::TargetTerminated(sig) => {
                    println!("\nTarget terminated with signal {}!", sig)
                }
                DisconnectReason::Kill => println!("\nGDB sent a kill command!"),
            },
            Err(e) => {
                if e.is_target_error() {
                    println!(
                        "target encountered a fatal error: {}",
                        e.into_target_error().unwrap()
                    )
                } else if e.is_connection_error() {
                    let (e, kind) = e.into_connection_error().unwrap();
                    println!("connection error: {:?} - {}", kind, e,)
                } else {
                    println!("gdbstub encountered a fatal error: {}", e)
                }
            }
        }
        Ok(exit_code)
    }
}


enum EventLoop {}

impl run_blocking::BlockingEventLoop for EventLoop {
    type Target = ZmuTarget;
    type Connection = TcpConnection;
    type StopReason = SingleThreadStopReason<u32>;

    #[allow(clippy::type_complexity)]
    fn wait_for_stop_reason(
        target: &mut ZmuTarget,
        conn: &mut Self::Connection,
    ) -> Result<
        run_blocking::Event<SingleThreadStopReason<u32>>,
        run_blocking::WaitForStopReasonError<
            <Self::Target as Target>::Error,
            <Self::Connection as Connection>::Error,
        >,
    > {

        let poll_incoming_data = || {
            // gdbstub takes ownership of the underlying connection, so the `borrow_conn`
            // method is used to borrow the underlying connection back from the stub to
            // check for incoming data.
            conn.peek().map(|b| b.is_some()).unwrap_or(true)
        };

        match target.run(poll_incoming_data) {
            SimulationRunEvent::IncomingData => {
                let byte = conn
                    .read()
                    .map_err(run_blocking::WaitForStopReasonError::Connection)?;
                Ok(run_blocking::Event::IncomingData(byte))
            }
            SimulationRunEvent::Event(event) => {
                use gdbstub::target::ext::breakpoints::WatchKind;

                // translate emulator stop reason into GDB stop reason
                let stop_reason = match event {
                    SimulationEvent::DoneStep => SingleThreadStopReason::DoneStep,
                    SimulationEvent::Halted => SingleThreadStopReason::Terminated(Signal::SIGSTOP),
                    SimulationEvent::Break => SingleThreadStopReason::SwBreak(()),
                    SimulationEvent::WatchWrite(addr) => SingleThreadStopReason::Watch {
                        tid: (),
                        kind: WatchKind::Write,
                        addr,
                    },
                    SimulationEvent::WatchRead(addr) => SingleThreadStopReason::Watch {
                        tid: (),
                        kind: WatchKind::Read,
                        addr,
                    },
                    SimulationEvent::Finalized(exit_code) => SingleThreadStopReason::Exited(exit_code as u8),
                };

                Ok(run_blocking::Event::TargetStopped(stop_reason))
            }
        }
    }

    fn on_interrupt(
        _target: &mut ZmuTarget,
    ) -> Result<Option<SingleThreadStopReason<u32>>, <ZmuTarget as Target>::Error> {
        // Because this emulator runs as part of the GDB stub loop, there isn't any
        // special action that needs to be taken to interrupt the underlying target. It
        // is implicitly paused whenever the stub isn't within the
        // `wait_for_stop_reason` callback.
        Ok(Some(SingleThreadStopReason::Signal(Signal::SIGINT)))
    }
}
