// extern crate slab;
// use self::slab::Slab;

use std::collections::HashMap;

use super::mio::tcp::TcpListener;
use super::mio::{Event, Poll, PollOpt, Ready, Token};
use std::io::Result as IoResult;

use super::connection::Connection;
use super::utils::assert_event_readiness;
use utils::result::DescribeErr;

const SERVER_TOKEN: Token = Token(0);

pub struct Server {
  socket: TcpListener,
  connections: HashMap<Token, Connection>,
  token_counter: usize,
}

impl Server {
  pub fn new(socket: TcpListener) -> Server {
    Server {
      socket,
      connections: HashMap::with_capacity(1024),
      token_counter: 0,
    }
  }

  pub fn register_into(&self, poll: &mut Poll) -> IoResult<()> {
    poll.register(
      &self.socket,
      SERVER_TOKEN,
      Ready::readable(),
      PollOpt::edge(),
    )
  }

  pub fn handle_event(
    &mut self,
    poll: &mut Poll,
    event: Event,
  ) -> IoResult<()> {
    match event.token() {
      SERVER_TOKEN => self.handle_server_event(poll, event),
      _ => self.handle_client_event(poll, event),
    }
  }

  pub fn handle_server_event(
    &mut self,
    poll: &mut Poll,
    event: Event,
  ) -> IoResult<()> {
    assert_event_readiness(event, Ready::readable());

    info!(target: "master::server", "accepting a socket");
    let (client_socket, client_addr) =
      self.socket.accept().describe_err("can't accept socket")?;
    info!(
      target: "master::server",
      "accepted a new socket from {}",
      client_addr,
    );

    let client_token = self.get_next_token();
    trace!(
      target: "master::server",
      "{:?} has been assigned to {}",
      client_token,
      client_addr,
    );

    trace!(target: "master::server", "creating connection");
    let mut connection =
      Connection::new(client_socket, client_addr, client_token);
    trace!(
      target: "master::server",
      "registering connection into event loop",
    );
    connection
      .register_into(poll)
      .describe_err("can't register client socket")?;

    self.connections.insert(client_token, connection);

    Ok(())
  }

  fn get_next_token(&mut self) -> Token {
    self.token_counter += 1;
    Token(self.token_counter)
  }

  pub fn handle_client_event(
    &mut self,
    poll: &mut Poll,
    event: Event,
  ) -> IoResult<()> {
    let token = event.token();

    let (is_closed, address) =
      if let Some(connection) = self.connections.get_mut(&token) {
        trace!(
          target: "master::server::connections",
          "found connection from {}",
          connection.address,
        );

        trace!(
          target: "master::server::connections",
          "current connection state = {:?}",
          connection.state,
        );
        connection
          .handle_event(poll, event)
          .describe_err(connection.address)?;
        trace!(
          target: "master::server::connections",
          "next connection state = {:?}",
          connection.state,
        );

        (connection.is_closed(), connection.address)
      } else {
        warn!(target: "master::server::connections", "unexpected event");
        return Ok(());
      };

    if is_closed {
      info!(
        target: "master::server::connections",
        "{} has disconnected",
        address,
      );
      self.connections.remove(&token);
    }

    Ok(())
  }
}
