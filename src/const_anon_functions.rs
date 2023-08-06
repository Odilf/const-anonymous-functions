/// A macro for creating const anonymous functions.
///
/// The syntax is closure syntax but with a couple of caveats. Namely:
/// - All types must be explicitly annotated, including the return type.
/// - The body must always be surrounded by braces.
///
/// See the [module-level documentation](crate) for more info.
#[macro_export]
macro_rules! caf {
	(|$($arg:tt: $arg_ty:tt),*| $(-> $return_ty:tt)? { $body:expr }) => {
		{
			const fn __anon_caf__($($arg: $arg_ty),*) $(-> $return_ty)? {
				$body
			}
			__anon_caf__
		}
	};
}
