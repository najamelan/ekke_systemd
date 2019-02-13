use typename          :: { TypeName               };
use actix             :: { prelude::*             };
use serde_derive      :: { Serialize, Deserialize };


use crate::Systemd;

#[ derive( Debug, Clone, Serialize, Deserialize, Message, TypeName ) ]
//
pub struct SendIndex {}



impl Handler<SendIndex> for Systemd
{
	type Result = ();

	fn handle( &mut self, _msg: SendIndex, _ctx: &mut Context<Self> ) -> Self::Result
	{
		println!( "Systemd: Received request for index" );
	}

}

