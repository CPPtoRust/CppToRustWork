// Using Namespace directives are not yet supported in this transpiler... Copying as it is
// using namespace std ;
#[derive(Default)]pub  struct  Bool_Array  {
 _size : u32 ,
 _iteration_number : u32 ,
 _storage_array :*mut u32 ,
 }
impl  Bool_Array  {
pub fn  new  (  size : u32  )  -> Bool_Array // Handling constructor initializer
{
Bool_Array {
 _size : size ,
 _iteration_number :1,
 _storage_array : u32  size };
 memset (  _storage_array  ,0 , size  * mem::size_of_val(( _storage_array [0])) );
 if  option [ DEBUG ]{
 fprintf (  stderr  ,"\nbool array size = %d, total bytes = %d\n" , self._size  , (  self._size  * mem::size_of_val(( _storage_array [0])) )  as  u32  );
}

/*
	This is a constructor method.
	Please appropriate members to the struct constructor as per your logic.
	Currently the constructor returns a struct with all the defaults for the data types in the struct.
*/
Bool_Array{..Default::default()}
}
pub fn  clear  ( &mut self ) { 
 if  self._iteration_number += 1 == 0{ 
 self._iteration_number  = 1;
 memset (  _storage_array  ,0 , self._size  * mem::size_of_val(( _storage_array [0])) );
 if  option [ DEBUG ]{ 
 fprintf (  stderr  ,"(re-initialized bool_array)\n" );
 fflush  (  stderr  ) ;
} 
} 
} 
pub fn  set_bit  ( &mut self,  index : u32  )  ->  bool { 
 if  _storage_array [ index ] ==  self._iteration_number {
 return true;
}
 else { 
 _storage_array  [  index  ]  =  self._iteration_number ;
 return false;
} 
} 
 }
