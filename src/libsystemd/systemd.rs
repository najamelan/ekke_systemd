use actix             :: { prelude::*                                        };
use failure           :: { ResultExt                                         };
use futures_util      :: { future::FutureExt, try_future::TryFutureExt };
use slog              :: { Logger, info, o                            };
use tokio_async_await :: { await                          };
use tokio_uds         :: { UnixStream                           };
use ekke_io           :: { IpcPeer, IpcMessage, Rpc, ResultExtSlog, Service, ConnID, SendRequest, MessageType };
use std::env::args;
use std::path::PathBuf;
use typename          :: { TypeName                                          };
use libekke::services::RegisterApplication;

//use crate::{ EkkeError };



mod send_index;
pub use send_index::*;


#[ derive( Debug, Clone, TypeName ) ]
//
pub struct Systemd
{
	pub log: Logger
}

impl Actor for Systemd
{
	type Context = Context<Self>;

	// Start the server
	// Register our services with the dispatcher
	//
	fn started( &mut self, ctx: &mut Self::Context )
	{
		let _our_address = ctx.address().clone();
		let log = self.log.clone();

		let rpc = Rpc::new( log.new( o!( "Actor" => "Rpc" ) ), crate::service_map ).start();

		self.register_service::<SendHtmlIndex>( &rpc, ctx );

		let program = async move
		{
			info!( log, "Ekke Systemd Starting up" );

			let arg        = args().nth( 2 ).expect( "No arguments passed in." );
			let sock_addr  = "\x00".to_string() + &arg;


			let connection = await!( UnixStream::connect( PathBuf::from( &sock_addr ) ) )

				.context( "Failed to connect to socket" ).unwraps( &log );


			let peer_log = log.new( o!( "Actor" => "IpcPeer" ) );


			let _ekke_server = IpcPeer::create( |ctx: &mut Context<IpcPeer<UnixStream>>|
			{
				IpcPeer::new( connection, rpc.recipient(), ctx.address(), peer_log )
			});


			let conn_id = ConnID::new();

			await!( rpc.send
			(
				SendRequest
				{
					IpcMessage::new
					(
						  "RegisterApplication".to_string()
						, RegisterApplication { conn_id, app_name: "Systemd".to_string() }
						, MessageType::SendRequest
						, conn_id
					)
				}

			)).unwraps( &log );

			Ok(())
		};

		Arbiter::spawn( program.boxed().compat() );
	}
}


