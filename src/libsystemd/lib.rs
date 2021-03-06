//! This is the actual functionality for the ekke framework server. The binary contains just a very basic main function. All functionality is exposed through this library so you could build against it if needed.
//
#![ feature( await_macro, async_await, futures_api, nll, stmt_expr_attributes, never_type ) ]

mod systemd;
mod errors;

pub use systemd::
{
	  Systemd
};



pub mod services
{
	pub use crate::systemd::SendHtmlIndex;
}


use
{
	crate   :: { services::*      } ,
	ekke_io :: { IpcMessage, Rpc  } ,
	actix   :: { Recipient        } ,
	slog    :: { Logger, error    } ,
};


pub(crate) fn service_map( rpc: &Rpc, log: Logger, msg: IpcMessage, ipc_peer: Recipient< IpcMessage > )
{
    match msg.service.as_ref()
    {
        "SendHtmlIndex" => rpc.deser_into::<SendHtmlIndex>( msg, ipc_peer ),
        _               => error!( &log, "Systemd: Received request for unknown service: {}", &msg.service ),
    }
}
