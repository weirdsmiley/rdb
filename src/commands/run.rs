use crate::debugger::Context;
use crate::utils::rdbError;
use std::error::Error;

pub(crate) struct RunTy {
    is_running: bool,
    pc: u32,
}

impl RunTy {
    pub(crate) fn new(run: bool, pc: u32) -> Result<Self, Box<dyn Error>> {
        Ok(RunTy {
            is_running: run,
            pc,
        })
    }

    pub(crate) fn run(&mut self) -> Result<&mut Self, Box<dyn Error>> {
        self.is_running = true;
        self.pc += 1;
        Ok(self)
    }
}

impl crate::commands::CmdTy for RunTy {
    type cmd = Option<String>;
    fn process(&mut self, cmd: Self::cmd) -> Result<(), Box<dyn Error>> {
        // start running the debugee until next interrupt is occurred
        println!("Running...");
        match self.run() {
            Ok(r) => Ok(()),
            // FIXME: Why should we match with **any** error?
            Err(_) => Err(Box::new(rdbError("unable to continue debugee".into()))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_RunTy() {
        let mut not_running = RunTy {
            is_running: false,
            pc: 0,
        };
        let first_run = RunTy {
            is_running: true,
            pc: 1,
        };
        match not_running.run() {
            Ok(x) => {
                assert!(not_running.is_running);
                assert!(not_running.pc == first_run.pc);
            }
            Err(_) => {
                eprintln!("test_RunTy failed");
            }
        }
    }
}
