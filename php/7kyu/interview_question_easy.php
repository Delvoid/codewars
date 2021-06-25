<?php
/*
You receive the name of a city as a string, and you need to return a string that shows how many times each letter shows up in the string by using asterisks (*).
For example:

"Chicago"  -->  "c:**,h:*,i:*,a:*,g:*,o:*"
As you can see, the letter c is shown only once, but with 2 asterisks.

The return string should include only the letters (not the dashes, spaces, apostrophes, etc). There should be no spaces in the output, and the different letters are separated by a comma (,) as seen in the example above.

Note that the return string must list the letters in order of their first appearance in the original string.

More examples:
"Bangkok"    -->  "b:*,a:*,n:*,g:*,k:**,o:*"
"Las Vegas"  -->  "l:*,a:**,s:**,v:*,e:*,g:*"
*/

function get_strings($city) {
  $city1 = strtolower(preg_replace("/[^a-zA-Z0-9]+/", "", $city));
  $lettersCount = [];
  $letters = [];
  $pattern = [];
  
  foreach(count_chars($city1, 1) as $i => $val)
  {
    $lettersCount += [chr($i) => $val];        
  }
  
  for($i = 0; $i < strlen($city1); $i++)
  {
    if(in_array($city1[$i], $letters)){
      continue;            
    }
    $letters[] = $city1[$i];        
    $pattern[] = $city1[$i] . ":". str_repeat('*', $lettersCount[$city1[$i]]);  
  }  
  return implode(',', $pattern);                                              
    
}

?>