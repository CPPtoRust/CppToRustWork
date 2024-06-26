#[derive(Default)]pub  struct  Node  {
 data : i32 ,
 next :*mut struct  struct  Node ,
 }
fn  printList  ( mut // Handling Pointers...
*mut n : struct  struct  Node  ) { 
 while  n  !=  NULL { 
 println! ( " {} " , n . data  );
 n  =  n . next ;
} 
} 
fn  main  (  ) { 
let mut // Handling Pointers...
*mut head : struct  struct  Node  =  NULL ;
let mut // Handling Pointers...
*mut second : struct  struct  Node  =  NULL ;
let mut // Handling Pointers...
*mut third : struct  struct  Node  =  NULL ;
 head  =  malloc (mem::size_of_val( struct  struct  Node )) as  struct  struct  Node *mut;
 second  =  malloc (mem::size_of_val( struct  struct  Node )) as  struct  struct  Node *mut;
 third  =  malloc (mem::size_of_val( struct  struct  Node )) as  struct  struct  Node *mut;
 head . data  = 1;
 head . next  =  second ;
 second . data  = 2;
 second . next  =  third ;
 third . data  = 3;
 third . next  =  NULL ;
 printList  (  head  ) ;
 return 0;
} 
