use super::mio::tcp::TcpStream;
use super::mio::{Event, Poll, PollOpt, Ready, Token};
use std::io::{Read, Result as IoResult, Write};
use std::net::SocketAddr;

use super::utils::assert_event_readiness;
use utils::result::DescribeErr;

pub struct Connection {
  socket: TcpStream,
  pub address: SocketAddr,
  token: Token,
  pub state: State,
  buffer: Box<[u8]>,
  buffer_bytes: usize,
}

#[derive(Debug)]
pub enum State {
  Reading,
  Writing,
  Closed,
}

impl Connection {
  pub fn new(
    socket: TcpStream,
    address: SocketAddr,
    token: Token,
  ) -> Connection {
    Connection {
      socket,
      address,
      token,
      state: State::Reading,
      buffer: Box::new([0; 1024]),
      buffer_bytes: 0,
    }
  }

  pub fn register_into(&mut self, poll: &mut Poll) -> IoResult<()> {
    poll.register(
      &self.socket,
      self.token,
      Ready::readable(),
      PollOpt::edge() | PollOpt::oneshot(),
    )
  }

  pub fn handle_event(
    &mut self,
    poll: &mut Poll,
    event: Event,
  ) -> IoResult<()> {
    match self.state {
      State::Reading => {
        assert_event_readiness(event, Ready::readable());
        self.read(poll)
      }
      State::Writing => {
        assert_event_readiness(event, Ready::writable());
        self.write(poll)
      }
      _ => unreachable!(),
    }
  }

  fn read(&mut self, poll: &mut Poll) -> IoResult<()> {
    match self
      .socket
      .read(&mut self.buffer)
      .describe_err("can't read from socket")?
    {
      0 => {
        self.state = State::Closed;
        Ok(())
      }
      n => {
        trace!("read {} bytes", n);
        self.buffer_bytes = n;
        self.state = State::Writing;

        self
          .reregister(poll)
          .describe_err("can't re-register socket")
      }
    }
  }

  fn write(&mut self, poll: &mut Poll) -> IoResult<()> {
    match self
      .socket
      .write(&self.buffer[..self.buffer_bytes])
      .describe_err("can't write to socket")?
    {
      n => {
        trace!("wrote {} bytes", n);
        self.state = State::Reading;

        self
          .reregister(poll)
          .describe_err("can't re-register socket")
      }
    }
  }

  fn reregister(&self, poll: &mut Poll) -> IoResult<()> {
    let interest = match self.state {
      State::Reading => Ready::readable(),
      State::Writing => Ready::writable(),
      _ => Ready::empty(),
    };

    poll.reregister(&self.socket, self.token, interest, PollOpt::oneshot())
  }

  pub fn is_closed(&self) -> bool {
    match self.state {
      State::Closed => true,
      _ => false,
    }
  }
}
