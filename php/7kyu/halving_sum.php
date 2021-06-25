<?php

/*
Given a positive integer n, calculate the following sum:

n + n/2 + n/4 + n/8 + ...
All elements of the sum are the results of integer division.

Example
25  =>  25 + 12 + 6 + 3 + 1 = 47
*/
function halvingSum($n) {
  $sum = $n;
  $i = 2;
  $a = $n;
  while($a > 1 )
  {    
    $a = floor($n/$i);    
    $i = $i*2;
    $sum += $a;
    
  }
  
  return (int) $sum;

}
?>