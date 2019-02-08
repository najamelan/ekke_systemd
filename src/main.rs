#![ feature( await_macro, async_await, futures_api ) ]

use serde_cbor::from_slice as des;

use libekke::services::RegisterApplication;
use tokio::codec::Decoder;

use tokio::codec::Framed;

use tokio::prelude::stream::{ SplitStream};

use tokio::prelude::*;


use tokio_serde_cbor::Codec;
use std::process::exit;
use futures_util::{future::FutureExt, try_future::TryFutureExt};
use std::env::args;

use tokio_uds::UnixStream;
use tokio_async_await::await;

use ekke_io::*;

use std::path::PathBuf;

use actix::prelude::*;

fn main()
{
	System::run( move ||	{ Arbiter::spawn( async move
	{
		println!( "PeerB: Starting" );

		// for argument in args()
		// {
		// 	println!( "PeerB: Argument passed on cli: {}", argument );
		// }

		let sock_addr = args().nth( 2 ).expect( "No arguments passed in." );

		println!( "PeerB: socket addres set to: {:?}", sock_addr );

		let socket = match await!( UnixStream::connect( PathBuf::from( &sock_addr ) ) )
		{
			Ok ( cx  ) => { cx },
			Err( err ) => { eprintln!( "{}", err ); exit( 1 ); }
		};

		let codec: Codec<IpcMessage, IpcMessage>  = Codec::new().packed( true );

		let (mut sink, stream) = codec.framed( socket ).split();


		tokio::spawn_async( async move
		{
			await!( listen( stream ) );
		} );


		let write   = IpcMessage::new( "RegistertApplication".into(), RegisterApplication{ app_name: "PeerB".to_string() } );


		match await!( sink.send_async( write ) )
		{
			Ok (_) => { println! ( "PeerB: successfully wrote to stream"       ); },
			Err(e) => { eprintln!( "PeerB: failed to write to stream: {:?}", e ); }
		}


		// System::current().stop();
		println!( "PeerB: Shutting down" );

		Ok(())

	}.boxed().compat())});
}


/// Will listen to a connection and send all incoming messages to the dispatch.
///
#[ inline ]
//
async fn listen( mut stream: SplitStream<Framed<UnixStream, Codec<IpcMessage, IpcMessage>>> )
{
	loop
	{
		let option: Option< Result< IpcMessage, _ > > = await!( stream.next() );

		let frame = match option
		{
			Some( result ) =>
			{
				match result
				{
					Ok ( frame ) => frame,
					Err( error ) =>
					{
						eprintln!( "Error extracting IpcMessage from stream" );
						eprintln!( "{:#?}", error );
						continue;
					}
				}
			},

			None => return     // Disconnected
		};

		let de: String = des( &frame.payload ).expect( "Failed to deserialize into String" );
		println!( "PeerB received: {}: {}", frame.service, de );
	}
}
