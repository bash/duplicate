#[duplicate::duplicate(
	name;
	[SomeName];
)]//duplicate_end
pub struct name();
//item_end

#[duplicate::duplicate(
	name		member;
	[SomeName]	[SomeMember]
)]//duplicate_end
pub struct name(member);
//item_end

#[duplicate::duplicate(
	name		member;
	[SomeName]	[SomeMember];
	[SomeName2]	[SomeMember2];
)]//duplicate_end
pub struct name(member);
//item_end

#[duplicate::duplicate(
	module ;
	[ mod1 ];
	[ mod2 ]
)]//duplicate_end
mod module {
	use super::*;
	
	#[duplicate::duplicate(
		name		member;
		[SomeName]	[SomeMember];
		[SomeName2]	[SomeMember2];
	)]//duplicate_end
	pub struct name(member);
	//item_end
}
//item_end

// Test substitution that includes braces
#[duplicate::duplicate(
	fn_name 		var;
	[ fn_name_1 ]	[ Struct() ];
	[ fn_name_2 ]	[ array[4] ];
	[ fn_name_3 ]	[ Struct{} ];
)]//duplicate_end
fn fn_name() {
	let _ = var;
}
//item_end