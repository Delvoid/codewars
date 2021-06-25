<?php
/*
Given an array of integers, find the one that appears an odd number of times.
There will always be only one integer that appears an odd number of times.
*/

function findIt(array $seq) : int
{
    $values = array_count_values($seq);
    foreach($values as $v => $c){
      if($c % 2  != 0){
        return $v;
      }
    }
}

?>