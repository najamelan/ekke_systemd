use actix             :: { prelude::*                                        };
use failure           :: { ResultExt                                         };
use futures_util      :: { future::FutureExt, try_future::TryFutureExt       };
use slog              :: { Logger, info, o                                   };
use tokio_async_await :: { await                                             };
use tokio_uds         :: { UnixStream                                        };
use std               :: { env::args, path::PathBuf                          };
use typename          :: { TypeName                                          };
use libekke::services :: { RegisterApplication, RegisterApplicationResponse  };

use ekke_io::
{
	  IpcPeer
	, IpcMessage
	, Rpc
	, ResultExtSlog
	, RegisterServiceMethod
	, ConnID
	, SendRequest
	, MessageType
};

//use crate::{ EkkeError };


mod     send_index   ;
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
		let rpc2 = rpc.clone();

		self.register_service::<SendHtmlIndex>( &rpc, ctx );

		let program = async move
		{
			info!( log, "Ekke Systemd Starting up" );

			let arg        = args().nth( 2 ).expect( "No arguments passed in." );
			let sock_addr  = "\x00".to_string() + &arg;


			let connection = await!( UnixStream::connect( PathBuf::from( &sock_addr ) ) )

				.context( "Failed to connect to socket" ).unwraps( &log );


			let peer_log = log.new( o!( "Actor" => "IpcPeer" ) );



			let ekke_server = IpcPeer::create( |ctx: &mut Context<IpcPeer<UnixStream>>|
			{
				IpcPeer::new( connection, rpc, ctx.address(), peer_log )
			});


			let conn_id = ConnID::new();

			let response = await!( rpc2.send
			(
				SendRequest
				{
					ipc_peer: ekke_server.recipient(),

					ipc_msg: IpcMessage::new
					(
						  "RegisterApplication".to_string()
						, RegisterApplication { conn_id, app_name: "Systemd".to_string() }
						, MessageType::SendRequest
						, conn_id
					)
				}

			)).unwraps( &log );

			let resp: RegisterApplicationResponse = Rpc::deserialize2( response.ipc_msg.payload ).unwrap();

			info!( log, "Systemd: Received response for RegisterApplication: {}", resp.response );

			Ok(())
		};

		Arbiter::spawn( program.boxed().compat() );
	}
}
