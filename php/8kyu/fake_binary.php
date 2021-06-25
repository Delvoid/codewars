<?php
/*
Given a string of digits, you should replace any digit below 5 with '0' and any digit 5 and above with '1'. Return the resulting string.
*/

function fake_bin(string $s): string {
  $output = "";
  for($i = 1; $i <= strlen($s); $i++){       
    $output .= $s[$i-1] < 5 ? 0 : 1;
  }
  return $output;
}
?>