use std::io;

use super::mio::tcp::{TcpListener, TcpStream};
use super::mio::util::Slab;
use super::mio::{EventLoop, EventSet, Handler, PollOpt, Token};
use super::mio::{TryRead, TryWrite};

use super::state::State;

pub type MyEventLoop = EventLoop<MyHandler>;

const SERVER_TOKEN: Token = Token(0);

pub struct MyHandler {
  server_socket: TcpListener,
  connections: Slab<Connection>,
}

impl MyHandler {
  pub fn new(server_socket: TcpListener) -> MyHandler {
    MyHandler {
      server_socket: server_socket,
      // Token `0` is reserved for the server socket. Tokens 1+ are used for
      // client connections. The slab is initialized to return Tokens
      // starting at 1.
      connections: Slab::new_starting_at(Token(1), 1024),
    }
  }

  pub fn register_into(&self, event_loop: &mut MyEventLoop) -> io::Result<()> {
    event_loop.register(
      &self.server_socket,
      SERVER_TOKEN,
      EventSet::readable(),
      PollOpt::edge(),
    )
  }
}

impl Handler for MyHandler {
  type Timeout = ();
  type Message = ();

  fn ready(
    &mut self,
    event_loop: &mut MyEventLoop,
    token: Token,
    events: EventSet,
  ) {
    match token {
      SERVER_TOKEN => {
        println!("the server socket is ready to accept a connection");
        match self.server_socket.accept() {
          Ok(Some((socket, _addr))) => {
            println!("accepted a new client socket");

            if self.connections.has_remaining() {
              // this will fail when the connection cap is reached
              let client_token = self
                .connections
                .insert_with(|conn_token| Connection::new(socket, conn_token))
                .unwrap();
              let client_connection = &self.connections[client_token];

              // register the connection with the event loop
              event_loop
                .register(
                  &client_connection.socket,
                  client_token,
                  EventSet::readable(),
                  PollOpt::edge() | PollOpt::oneshot(),
                )
                .unwrap();
            }
          }
          Ok(None) => {
            println!("the server socket wasn't actually ready");
          }
          Err(e) => {
            println!(
              "encountered error while accepting connection; err={:?}",
              e
            );
            event_loop.shutdown();
          }
        }
      }
      _ => {
        let is_closed = {
          let client_connection = &mut self.connections[token];
          client_connection.ready(event_loop, events);
          client_connection.is_closed()
        };

        // If handling the event resulted in a closed socket, then remove the
        // socket from the Slab (this will free all resources)
        if is_closed {
          self.connections.remove(token);
        }
      }
    }
  }
}

const MAX_LINE: usize = 128;

#[derive(Debug)]
struct Connection {
  socket: TcpStream,
  token: Token,
  state: State,
}

impl Connection {
  fn new(socket: TcpStream, token: Token) -> Connection {
    Connection {
      socket: socket,
      token: token,
      state: State::Reading(Vec::with_capacity(MAX_LINE)),
    }
  }

  fn ready(&mut self, event_loop: &mut MyEventLoop, events: EventSet) {
    match self.state {
      State::Reading(..) => {
        assert!(
          events.is_readable(),
          "unexpected events; events={:?}",
          events
        );
        self.read(event_loop)
      }
      State::Writing(..) => {
        assert!(
          events.is_writable(),
          "unexpected events; events={:?}",
          events
        );
        self.write(event_loop)
      }
      _ => unimplemented!(),
    }
  }

  fn read(&mut self, event_loop: &mut MyEventLoop) {
    match self.socket.try_read_buf(self.state.mut_read_buf()) {
      Ok(Some(0)) => {
        self.state = State::Closed;
      }
      Ok(Some(n)) => {
        println!("read {} bytes", n);

        // Look for a new line. If a new line is received, then the
        // state is transitioned from `Reading` to `Writing`.
        self.state.try_transition_to_writing();

        // Re-register the socket with the event loop. The current
        // state is used to determine whether we are currently reading
        // or writing.
        self.reregister(event_loop);
      }
      Ok(None) => {
        self.reregister(event_loop);
      }
      Err(e) => {
        panic!("got an error trying to read; err={:?}", e);
      }
    }
  }

  fn write(&mut self, event_loop: &mut MyEventLoop) {
    // TODO: handle error
    match self.socket.try_write_buf(self.state.mut_write_buf()) {
      Ok(Some(_)) => {
        // If the entire line has been written, transition back to the
        // reading state
        self.state.try_transition_to_reading();

        // Re-register the socket with the event loop.
        self.reregister(event_loop);
      }
      Ok(None) => {
        // The socket wasn't actually ready, re-register the socket
        // with the event loop
        self.reregister(event_loop);
      }
      Err(e) => {
        panic!("got an error trying to write; err={:?}", e);
      }
    }
  }

  fn reregister(&self, event_loop: &mut MyEventLoop) {
    event_loop
      .reregister(
        &self.socket,
        self.token,
        self.state.event_set(),
        PollOpt::oneshot(),
      )
      .unwrap();
  }

  fn is_closed(&self) -> bool {
    match self.state {
      State::Closed => true,
      _ => false,
    }
  }
}
