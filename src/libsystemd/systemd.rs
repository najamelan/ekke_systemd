use actix             :: { prelude::*                                        };
use failure           :: { ResultExt                                         };
use futures_util      :: { future::FutureExt, try_future::TryFutureExt };
use slog              :: { Logger, info, o                            };
use tokio_async_await :: { await                          };
use tokio_uds         :: { UnixStream                           };
use ekke_io           :: { IpcPeer, IpcMessage, Dispatcher, ResultExtSlog, Service };
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

		let dispatcher = Dispatcher::new( log.new( o!( "Actor" => "Dispatcher" ) ), crate::service_map ).start();

		self.register_service::<SendIndex>( &dispatcher, ctx );

		let program = async move
		{
			info!( log, "Ekke Systemd Starting up" );

			let arg        = args().nth( 2 ).expect( "No arguments passed in." );
			let sock_addr  = "\x00".to_string() + &arg;


			let connection = await!( UnixStream::connect( PathBuf::from( &sock_addr ) ) )

				.context( "Failed to connect to socket" ).unwraps( &log );


			let peer_log = log.new( o!( "Actor" => "IpcPeer" ) );


			let ekke_server = IpcPeer::create( |ctx: &mut Context<IpcPeer>|
			{
				IpcPeer::new( connection, dispatcher.recipient(), ctx.address(), peer_log )
			});


			await!( ekke_server.send
			(
				IpcMessage::new
				(
					  "RegisterApplication".to_string()
					, RegisterApplication { app_name: "Systemd".to_string() }
				)

			)).unwraps( &log );

			Ok(())
		};

		Arbiter::spawn( program.boxed().compat() );
	}
}


