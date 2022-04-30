extern crate mio;
use self::mio::tcp::TcpListener;
use self::mio::{Events, Poll};

use std::io::Result as IoResult;
use std::net::SocketAddr;

mod connection;
mod server;
mod utils;

use utils::result::DescribeErr;

pub fn listen(port: u16) -> IoResult<()> {
  let address = SocketAddr::from(([0, 0, 0, 0], port));
  info!(target: "master", "starting master server");

  trace!(target: "master", "binding server socket to {}", address);
  let server_socket =
    TcpListener::bind(&address).describe_err("can't bind server socket")?;

  let server = server::Server::new(server_socket);

  trace!(target: "master", "creating event loop");
  let mut event_loop =
    EventLoop::new(server).describe_err("can't create event loop")?;

  info!(target: "master", "server is listening on port {}", port);
  event_loop.run()
}

struct EventLoop {
  poll: Poll,
  events: Events,
  server: server::Server,
}

impl EventLoop {
  fn new(server: server::Server) -> IoResult<EventLoop> {
    trace!(target: "master::event_loop", "creating poll");
    let mut poll = Poll::new().describe_err("can't create poll")?;

    trace!(target: "master::event_loop", "creating events collection");
    let events = Events::with_capacity(1024);

    trace!(target: "master::event_loop", "registering server");
    server
      .register_into(&mut poll)
      .describe_err("can't register server")?;

    Ok(EventLoop {
      poll,
      events,
      server,
    })
  }

  fn run(&mut self) -> IoResult<()> {
    loop {
      trace!(target: "master::event_loop", "tick");
      self.tick()?;
    }
  }

  fn tick(&mut self) -> IoResult<()> {
    let events_count = self
      .poll
      .poll(&mut self.events, None)
      .describe_err("can't get events")?;
    trace!(target: "master::event_loop", "events_count = {}", events_count);

    for event in self.events.iter() {
      trace!(target: "master::event_loop", "event = {:?}", event);

      if let Err(error) = self.server.handle_event(&mut self.poll, event) {
        return Err(error);
      }
    }

    Ok(())
  }
}
