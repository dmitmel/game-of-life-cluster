extern crate mio;

use self::mio::tcp::TcpListener;
use std::io::Result as IoResult;
use std::net::SocketAddr;

mod handler;
mod state;

use utils::result::DescribeErr;

pub fn listen(port: u16) -> IoResult<()> {
  let address = SocketAddr::from(([0, 0, 0, 0], port));
  let server_socket =
    TcpListener::bind(&address).describe_err("can't bind server socket")?;

  let mut event_loop =
    handler::MyEventLoop::new().describe_err("can't create event loop")?;

  let mut handler = handler::MyHandler::new(server_socket);

  handler
    .register_into(&mut event_loop)
    .describe_err("can't register server socket in the event loop")?;

  println!("listening on port {}", port);
  event_loop
    .run(&mut handler)
    .describe_err("can't start event loop")?;

  Ok(())
}
