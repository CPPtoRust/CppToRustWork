/*************************************************************************
		* This file was generated by CRUST by an automated semantics preserving
		* translation from C/C++ to Rust
		* CRUST isn't perfect and may require manual editing
		* Check warnings and errors and refer to the official Rust Documentation
		************************************************************************/
		
		/*Crust with Strict Mode enabled, declares all variables as immutable.
		* If you are mutating the below variable anywhere in program, please change the declaration statement as
		* let mut var_name:type=init_val;
		**/
		static n : i64 ;
		
		// catalan(n) is sum of
		// catalan(i)*catalan(n-i-1)
		
		/*Crust with Strict Mode enabled, declares all variables as immutable.
		* If you are mutating the below variable anywhere in program, please change the declaration statement as
		* let mut var_name:type=init_val;
		**/
		static res : i64 = 0; ;
		
		/*Crust with Strict Mode enabled, declares all variables as immutable.
		* If you are mutating the below variable anywhere in program, please change the declaration statement as
		* let mut var_name:type=init_val;
		**/
		static i : i32 = 0; ; while i < n {
			res += catalan ( i ) * catalan ( n - i - 1 ) ; i +=1 ; }