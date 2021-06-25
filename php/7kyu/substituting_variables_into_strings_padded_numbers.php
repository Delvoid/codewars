<?php
/*
Complete the solution so that it returns a formatted string. The return value should equal "Value is VALUE" where value is a 5 digit padded number.
Example:
solution(5) // should return "Value is 00005"
*/
function solution($value){
  // Make it green, than make it clean :)
  return "Value is ".str_pad($value, 5, 0 , STR_PAD_LEFT );
;
};
?>