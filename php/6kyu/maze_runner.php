<?php
/*
Introduction
Welcome Adventurer. Your aim is to navigate the maze and reach the finish point without touching any walls. Doing so will kill you instantly!
Task
You will be given a 2D array of the maze and an array of directions. Your task is to follow the directions given. If you reach the end point before all your moves have gone, you should return Finish. If you hit any walls or go outside the maze border, you should return Dead. If you find yourself still in the maze after using all the moves, you should return Lost.
The Maze array will look like

maze = [[1,1,1,1,1,1,1],
        [1,0,0,0,0,0,3],
        [1,0,1,0,1,0,1],
        [0,0,1,0,0,0,1],
        [1,0,1,0,1,0,1],
        [1,0,0,0,0,0,1],
        [1,2,1,0,1,0,1]]
..with the following key

      0 = Safe place to walk
      1 = Wall
      2 = Start Point
      3 = Finish Point
  direction = ["N","N","N","N","N","E","E","E","E","E"] == "Finish"
Rules
1. The Maze array will always be square i.e. N x N but its size and content will alter from test to test.
2. The start and finish positions will change for the final tests.
3. The directions array will always be in upper case and will be in the format of N = North, E = East, W = West and S = South.
4. If you reach the end point before all your moves have gone, you should return Finish.
5. If you hit any walls or go outside the maze border, you should return Dead.
6. If you find yourself still in the maze after using all the moves, you should return Lost.
*/
function maze_runner($maze, $directions): string {
  $n = count($maze[1]);
  $start = null;
  $finish = null;
  $pos = null;
  //Get start pos
  foreach($maze as $y => $row){
    foreach($row as $x => $r){
      if($r == 2){
        $start = [$y, $x];
        $pos = [$y, $x];
      }      
    }
  }  
  //move pos  
  foreach($directions as $dir){    
    $cur = move($pos, $dir);
    $moveTo = $maze[$cur[0]][$cur[1]];
    //check if out of bounds or hit a wall
    print($cur[1]);
    if($cur[0] < 0 || $cur[1] < 0 || $cur[0] >= $n || $cur[1] >= $n || $moveTo === 1){
      return "Dead";
    }
    //if not a wall, move pos
    if($moveTo != 1){
      $pos[0] = $cur[0];
      $pos[1] = $cur[1];  
    }
    //check if its the finish
    if($moveTo === 3){
      $finish = [$pos[0], $pos[1]];
      return "Finish";
    } 
  } 
  
  return 'Lost';
  
}
function move($pos, $dir)
{
  if($dir === "N"){
    $pos[0] = $pos[0]-1;
  }
  elseif($dir === "E"){
    $pos[1] = $pos[1]+1;
    
  }elseif($dir === "S"){    
    $pos[0] = $pos[0]+1;
    
  }elseif($dir === "W"){    
    $pos[1] = $pos[1]-1;
  }  
  return $pos;  
}

//Print maze
function printMaze($maze){
  $n = count($maze[1]);
  foreach($maze as $y => $row){
    foreach($row as $x => $r){
      print($r);
      if($x == $n -1)
      print("<br />");      
    }
  }  
}

?>