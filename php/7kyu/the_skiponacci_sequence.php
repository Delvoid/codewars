<?php
/*
Your task is to generate the Fibonacci sequence to n places, with each alternating value as "skip". For example:
"1 skip 2 skip 5 skip 13 skip 34"
Return the result as a string
You can presume that n is always a positive integer between (and including) 1 and 64.
*/
function skiponacci($n) {  
  $num1 = 0;
  $num2 = 1;
  for($i=0; $i < $n; $i++){
    $num2 = $num2 + $num1;
    $num1 = $num2 - $num1;

    $outcome[] = ($i % 2) ? 'skip' : $num1;
    
  }
  return implode(' ', $outcome);
    
}
?>