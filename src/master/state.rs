extern crate bytes;
use self::bytes::{Buf, Take};

use std::io::Cursor;
use std::mem;

use super::mio;

#[derive(Debug)]
pub enum State {
  Reading(Vec<u8>),
  Writing(Take<Cursor<Vec<u8>>>),
  Closed,
}

impl State {
  pub fn mut_read_buf(&mut self) -> &mut Vec<u8> {
    match *self {
      State::Reading(ref mut buf) => buf,
      _ => panic!("connection not in reading state"),
    }
  }

  fn read_buf(&self) -> &[u8] {
    match *self {
      State::Reading(ref buf) => buf,
      _ => panic!("connection not in reading state"),
    }
  }

  fn write_buf(&self) -> &Take<Cursor<Vec<u8>>> {
    match *self {
      State::Writing(ref buf) => buf,
      _ => panic!("connection not in writing state"),
    }
  }

  pub fn mut_write_buf(&mut self) -> &mut Take<Cursor<Vec<u8>>> {
    match *self {
      State::Writing(ref mut buf) => buf,
      _ => panic!("connection not in writing state"),
    }
  }

  // Looks for a new line, if there is one the state is transitioned to
  // writing
  pub fn try_transition_to_writing(&mut self) {
    if let Some(pos) = self.read_buf().iter().position(|b| *b == b'\n') {
      // First, remove the current read buffer, replacing it with an
      // empty Vec<u8>.
      let buf = mem::replace(self, State::Closed).unwrap_read_buf();

      // Wrap in `Cursor`, this allows Vec<u8> to act as a readable
      // buffer
      let buf = Cursor::new(buf);

      // Transition the state to `Writing`, limiting the buffer to the
      // new line (inclusive).
      *self = State::Writing(Take::new(buf, pos + 1));
    }
  }

  // If the buffer being written back to the client has been consumed, switch
  // back to the reading state. However, there already might be another line
  // in the read buffer, so `try_transition_to_writing` is called as a final
  // step.
  pub fn try_transition_to_reading(&mut self) {
    if !self.write_buf().has_remaining() {
      let cursor = mem::replace(self, State::Closed)
        .unwrap_write_buf()
        .into_inner();

      let pos = cursor.position();
      let mut buf = cursor.into_inner();

      // Drop all data that has been written to the client
      buf.drain(0..pos as usize);

      *self = State::Reading(buf);

      // Check for any new lines that have already been read.
      self.try_transition_to_writing();
    }
  }

  pub fn event_set(&self) -> mio::EventSet {
    match *self {
      State::Reading(..) => mio::EventSet::readable(),
      State::Writing(..) => mio::EventSet::writable(),
      _ => mio::EventSet::none(),
    }
  }

  fn unwrap_read_buf(self) -> Vec<u8> {
    match self {
      State::Reading(buf) => buf,
      _ => panic!("connection not in reading state"),
    }
  }

  fn unwrap_write_buf(self) -> Take<Cursor<Vec<u8>>> {
    match self {
      State::Writing(buf) => buf,
      _ => panic!("connection not in writing state"),
    }
  }
}
