<?php
/*
Count the number of occurrences of each character and return it as a list of tuples in order of appearance. For empty output return an empty list.
Example:
orderedCount("abracadabra") == [['a', 5], ['b', 2], ['r', 2], ['c', 1], ['d', 1]]
*/

function orderedCount($text) {
  if(empty($text)) return [];
  
  foreach(array_count_values(str_split($text)) as $char => $value){
    $output[] = [$char, $value];
  } 
  
  return $output;

}

?>