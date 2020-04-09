

fn main() {
   // let  x  = 5;
   // let x = 6; // we cannt even assign the same value again.
   let mut x = 5;
   println!("The value of x is : {} ", x);

   x = 8;
   println!("the value of x is : {}", x);

   println!("testing calling a method");
   shadow_variables_and_add_constants();
}

fn shadow_variables_and_add_constants() {
   const MAX_POINTS: u32 = 100_000;
   println!("the max point is  : {}", MAX_POINTS);

   let x = 5;

   let x = x + 1;

   let x  =  x * 2;


   println!("The value of x is : {}", x);

   let spaces = "    ";
   let spaces=  spaces.len();
   println!("the number of spaces is  : {}", spaces);
}

