use lib_core::model::user::{User, UserBmc, UserForCreate};
use lib_rpc_core::prelude::*;

pub fn rpc_router_builder() -> RouterBuilder {
	todo!()
    /*router_builder!(
		create_user,
		delete_user,
	)*/
}

generate_common_rpc_fns!(
    Bmc: UserBmc,
	Entity: User,
	ForCreate: UserForCreate,
	ForUpdate: UserForUpdate,
	Filter: UserFilter,
	Suffix: user
);
