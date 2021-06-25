<?php

/*
In this Kata, we are going to reverse a string while maintaining the spaces (if any) in their original place.

For example:

solve("our code") = "edo cruo"
-- Normal reversal without spaces is "edocruo". 
-- However, there is a space at index 3, so the string becomes "edo cruo"

solve("your code rocks") = "skco redo cruoy". 
solve("codewars") = "srawedoc"
More examples in the test cases. All input will be lower case letters and in some cases spaces.
*/

function solve(string $s): string {
  
  $length = strlen($s);
  $chars = preg_split('//', $s, -1, PREG_SPLIT_NO_EMPTY);
  $result = [];

  for($i = 0; $i < $length; $i++)
  {
    if($chars[$i] == ' ' or $chars[$i] == '  '){      
      $result[$i] = ' ';
    }
  }
  
  $j = $length -1;  
  for($i = 0; $i < strlen($s); $i++)
  {
    print($chars[$i]);
    if($chars[$i] != ' ')
    {      
      if($result[$j] == ' ')
      {
        $j--;        
      }
      
      $result[$j] = $chars[$i];
      $j--;     

    }    
  }    
  ksort($result);  
  return implode($result);   

}

?>