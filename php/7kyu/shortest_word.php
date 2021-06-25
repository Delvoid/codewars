<?php
/*
Simple, given a string of words, return the length of the shortest word(s).
String will never be empty and you do not need to account for different data types.
*/
function findShort($str){
  $words = explode(" ", $str);
  $length = [];
  foreach($words as $word){
    array_push($length, strlen($word));
  }
  
  return min($length);
}
?>