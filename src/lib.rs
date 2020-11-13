//! A pure [Rust](https://www.rust-lang.org)
//! [Modbus](https://en.wikipedia.org/wiki/Modbus) library
//! based on [tokio](https://tokio.rs).
//!
//! Modbus is based on a [master/slave](https://en.wikipedia.org/wiki/Master/slave_(technology))
//! model.
//! To avoid confusions with the tokio terminology the master is called *client*
//! and the slave is called *server* in this library.
//!
//! ## Features
//!
//! - pure Rust library
//! - async (non-blocking)
//! - sync (blocking)
//! - Modbus TCP
//! - Client & Server
//! - Open Source (MIT/Apache-2.0)
//!
//! # Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! tokio-modbus = "*"
//! ```
//! If you like to use Modbus TCP only:
//!
//! ```toml
//! [dependencies]
//! tokio-modbus = { version = "*", default-features = false, features = ["tcp"] }
//! ```
//!
//! If you like to build a TCP server:
//!
//! ```toml
//! [dependencies]
//! tokio-modbus = { version = "*", default-features = false, features = ["tcp", "server"] }
//! ```
//!
//! # Examples
//!
//! ## TCP client
//!
//! ```rust,no_run
//! #[cfg(feature = "tcp")]
//! #[tokio::main]
//! pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     use std::future::Future;
//!     use tokio::runtime::Runtime;
//!     use tokio_modbus::prelude::*;
//!
//!     let socket_addr = "192.168.0.222:502".parse().unwrap();
//!
//!     let mut ctx = tcp::connect(socket_addr).await?;
//!     let data = ctx.read_input_registers(0x1000, 7).await?;
//!     println!("Response is '{:?}'", data);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Sync TCP client
//!
//! ```rust,no_run
//! #[cfg(all(feature = "tcp", feature = "sync"))]
//! pub fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     use tokio_modbus::prelude::*;
//!
//!     let socket_addr = "192.168.0.222:502".parse()?;
//!     let mut client = client::sync::tcp::connect(socket_addr)?;

//!     let data = client.read_input_registers(0x1000, 7)?;
//!     println!("Response is '{:?}'", data);
//!
//!     Ok(())
//! }
//! ```
//!
//!
//! More examples can be found in the [examples](https://github.com/slowtec/tokio-modbus/tree/master/examples) folder.
//!
//! # Protocol-Specification
//!
//! - [MODBUS Application Protocol Specification v1.1b3 (PDF)](http://modbus.org/docs/Modbus_Application_Protocol_V1_1b3.pdf)
//! - [MODBUS over serial line specification and implementation guide v1.02 (PDF)](http://modbus.org/docs/Modbus_over_serial_line_V1_02.pdf)
//! - [MODBUS Messaging on TCP/IP Implementation Guide v1.0b (PDF)](http://modbus.org/docs/Modbus_Messaging_Implementation_Guide_V1_0b.pdf)

pub mod prelude;

pub mod client;
pub mod slave;

#[cfg(feature = "server")]
pub mod server;

mod codec;
mod frame;
mod service;
