#[cfg(not(test))]
pub use std::sync::mpsc;
#[cfg(test)]
pub mod mpsc {
    use std::io::Error;
    pub use std::sync::mpsc::{RecvError, SendError, TryRecvError};

    static mut ID: usize = 0;

    #[derive(Clone, Debug)]
    pub struct Sender<T> {
        pub outputs: Vec<T>,
        id: usize,
    }

    impl<T> PartialEq for Sender<T> {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl<T> Sender<T> {
        pub fn send(&mut self, t: T) -> Result<(), SendError<T>>
        where
            T: PartialEq<T>,
        {
            if self.outputs.pop().unwrap() == t {
                Ok(())
            } else {
                Err(SendError(t))
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Receiver<T> {
        pub inputs: Vec<T>,
        id: usize,
    }

    impl<T> PartialEq for Receiver<T> {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl<T> Receiver<T> {
        pub fn recv(&mut self) -> Result<T, RecvError> {
            if self.inputs.is_empty() {
                Err(RecvError)
            } else {
                Ok(self.inputs.pop().unwrap())
            }
        }

        pub fn try_recv(&self) -> Result<T, TryRecvError> {
            todo!()
        }
    }

    pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
        unsafe {
            ID += 1;
            (
                Sender {
                    outputs: vec![],
                    id: ID,
                },
                Receiver {
                    inputs: vec![],
                    id: ID,
                },
            )
        }
    }

    pub fn channel_with_checks<T>(outputs: Vec<T>, inputs: Vec<T>) -> (Sender<T>, Receiver<T>) {
        unsafe {
            ID += 1;
            (Sender { outputs, id: ID }, Receiver { inputs, id: ID })
        }
    }
}
