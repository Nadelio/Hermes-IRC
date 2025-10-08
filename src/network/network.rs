use crate::tui::tui::{ERROR, RESET, SYSTEM};
use rustls::ClientConfig;
use rustls::{ClientConnection, StreamOwned};
use std::collections::HashSet;
use std::convert::TryFrom;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const PORT: u32 = 6667;

pub fn network_info() -> String {
	format!("$ {SYSTEM}[Server] Port: {PORT}, IP: Local Host{RESET}\n")
}

/// Parses an IRC message according to RFC 1459 and RFC 2812
/// IRC Messages are formatted as such: [:<prefix>\ ] <command> <params>* [:<trailing>] <CRLF>
/// Returns (prefix, command, parameters)
/// parameters is a Vec<String> = vec![params, trailing?]
fn parse_line(line: &str) -> (Option<String>, String, Vec<String>) {
	let mut rest = line;
	let mut prefix = None;

	// parse prefix [:<prefix>\ ]
	if line.starts_with(':')
		&& let Some(next_space) = line.find(' ')
	{
		prefix = Some(line[1..next_space].to_string()); // separate prefix from the rest of the
																									// code
		rest = &line[next_space + 1..]; // get the rest of the line after the prefix
	}

	let mut params = Vec::new();
	let command;

	// parse trailing [:<trailing>]
	if let Some(trailing) = rest.find(" :") {
		let before = &rest[..trailing]; // get everything before the trailing
		let trailing_content = &rest[trailing + 2..]; // add 2 to skip both the space and the colon and
																								// exclude them from the content following the
																								// trailing identifier
																								// split up `before` and add it to the params vector
		let mut tokens: Vec<&str> = before.split_whitespace().collect();
		command = tokens.remove(0).to_string(); // get the command
		params.extend(tokens.iter().map(|s| s.to_string())); // add the rest of the tokens to params
		params.push(trailing_content.to_string());
	} else {
		// if no trailing, exclude processing it
		let mut tokens: Vec<&str> = rest.split_whitespace().collect();
		command = tokens.remove(0).to_string();
		params.extend(tokens.iter().map(|s| s.to_string()));
	}

	(prefix, command, params)
}

enum Connection {
	TCP(TcpStream),
	TLS(Box<StreamOwned<ClientConnection, TcpStream>>),
}

impl Write for Connection {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		match self {
			Connection::TCP(stream) => stream.write(buf),
			Connection::TLS(stream) => stream.write(buf),
		}
	}

	fn flush(&mut self) -> std::io::Result<()> {
		match self {
			Connection::TCP(stream) => stream.flush(),
			Connection::TLS(stream) => stream.flush(),
		}
	}
}

impl std::io::Read for Connection {
	fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
		match self {
			Connection::TCP(stream) => stream.read(buf),
			Connection::TLS(stream) => stream.read(buf),
		}
	}
}

/// IRC Client structure
/// Handles connections, message sending/receiving, and other user interactions
struct IRCClient {
	server: String,
	port: u16,
	nick: String,
	username: String,
	realname: String,
	use_tls: bool,
	sender: Option<Sender<String>>,
	registered: std::sync::Arc<std::sync::atomic::AtomicBool>, // needs to be atomic since we are
	// accessing from several threads
	channel_list: Arc<Mutex<HashSet<String>>>, // tracker list for currently joined channels
}

impl IRCClient {
	fn new(server: &str, port: u16, nick: &str, user: &str, real: &str, tls: bool) -> IRCClient {
		IRCClient {
			server: server.to_string(),
			port,
			nick: nick.to_string(),
			username: user.to_string(),
			realname: real.to_string(),
			use_tls: tls,
			sender: None, // initialized elsewhere
			registered: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
			channel_list: Arc::new(Mutex::new(HashSet::new())),
		}
	}

	fn create_tls_config() -> Result<Arc<ClientConfig>, Box<dyn std::error::Error>> {
		let mut root_store = rustls::RootCertStore::empty();

		root_store.add_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.iter().map(|ta| {
			rustls::OwnedTrustAnchor::from_subject::spki_name_constraints(
				ta.subject,
				ta.spki,
				ta.name_constraints,
			)
		}));

		let config = ClientConfig::builder()
			.with_safe_defaults()
			.with_root_certificates(root_store)
			.with_no_client_auth();

		Ok(Arc::new(config));
	}

	/// Create a connection between the local client and a remote endpoint (can be client or server)
	fn create_connection(&self) -> std::io::Result<Connection> {
		if self.use_tls {
			// builds TLS connection
			let tcp_stream = TcpStream::connect((self.server.as_str(), self.port))?;

			let config = Self::create_tls_config().map_err(|e| {
				std::io::Error::other(format!("{ERROR}[Error] TLS Config Error: {e}{RESET}"))
			})?;

			let server_name = rustls::ServerName::try_from(self.server.as_str()).map_err(|e| {
				std::io::Error::new(
					std::io::ErrorKind::InvalidInput,
					format!("{ERROR}[Error] Invalid Server Name: {e}{RESET}"),
				)
			})?;

			let client = ClientConnection::new(config, server_name).map_err(|e| {
				std::io::Error::other(format!("{ERROR}[Error] TLS Connection Error: {e}{RESET}"))
			})?;

			Ok(Connection::TLS(Box::new(StreamOwned::new(
				client, tcp_stream,
			))))
		} else {
			Ok(Connection::TCP(TcpStream::connect((
				self.server.as_str(),
				self.port,
			))?))
		}
	}

	fn connect(&mut self) -> std::io::Result<()> {}
}
