// These tests ensure that the short syntax works if it
// was produced from the expansion of a macro_rules macro.
//
// Each test consists of a macro_rules declaration which uses
// some specific macro variable type to no_features to the duplicate invocation.
// Then the created macro is invoked.

macro_rules! test_ident_from_macro_variable{
	{ $name:ident } => {
		#[duplicate::duplicate(
			$name;	[SomeName1]
		)]//duplicate_end
		pub struct $name();
		//item_end
	}
}
test_ident_from_macro_variable!(name);

macro_rules! test_2_idents_from_macro_variable{
	{ $($idents:ident)* } => {
		#[duplicate::duplicate(
			$($idents)*;
			[SomeName2] [SomeMember2]
		)]//duplicate_end
		pub struct name(member);
		//item_end
	}
}
test_2_idents_from_macro_variable!(name member);

macro_rules! test_ident_from_macro_path_variable{
	{ $name:path } => {
		#[duplicate::duplicate(
			$name;	[SomeMember3]
		)]//duplicate_end
		pub struct SomeName3($name);
		//item_end
	}
}
test_ident_from_macro_path_variable!(name);

macro_rules! test_ident_from_macro_expr_variable{
	{ $name:expr } => {
		#[duplicate::duplicate(
			$name;	[SomeValue4]
		)]//duplicate_end
		const SomeName4: () = $name;
		//item_end
	}
}
test_ident_from_macro_expr_variable!(name);

macro_rules! test_ident_from_macro_type_variable{
	{ $name:ty } => {
		#[duplicate::duplicate(
			$name;	[SomeMember5]
		)]//duplicate_end
		pub struct SomeName5($name);
		//item_end
	}
}
test_ident_from_macro_type_variable!(name);

macro_rules! test_ident_from_macro_pattern_variable{
	{ $name:pat } => {
		#[duplicate::duplicate(
			$name;	[SomeName6]
		)]//duplicate_end
		fn some_fn6(){
			let $name;
		}
		//item_end
	}
}
test_ident_from_macro_pattern_variable!(name);

macro_rules! test_ident_from_macro_statement_variable{
	{ $name:stmt } => {
		#[duplicate::duplicate(
			$name;	[SomeName7]
		)]//duplicate_end
		fn some_fn7(){
			$name;
		}
		//item_end
	}
}
test_ident_from_macro_statement_variable!(name);

macro_rules! test_ident_from_macro_token_tree_variable{
	{ $name:tt } => {
		#[duplicate::duplicate(
			$name;	[SomeName8]
		)]//duplicate_end
		pub struct $name();
		//item_end
	}
}
test_ident_from_macro_token_tree_variable!(name);