<?php
/*
Snakes and Ladders is an ancient Indian board game regarded today as a worldwide classic. It is played between two or more players on a gameboard having numbered, gridded squares. A number of "ladders" and "snakes" are pictured on the board, each connecting two specific board squares. (Source Wikipedia)

Task
Your task is to make a simple class called SnakesLadders. The test cases will call the method play(die1, die2) independantly of the state of the game or the player turn. The variables die1 and die2 are the die thrown in a turn and are both integers between 1 and 6. The player will move the sum of die1 and die2.
The Board - see image

Rules
1.  There are two players and both start off the board on square 0.
2.  Player 1 starts and alternates with player 2.
3.  You follow the numbers up the board in order 1=>100
4.  If the value of both die are the same then that player will have another go.
5.  Climb up ladders. The ladders on the game board allow you to move upwards and get ahead faster. If you land exactly on a square that shows an image of the bottom of a ladder, then you may move the player all the way up to the square at the top of the ladder. (even if you roll a double).
6.  Slide down snakes. Snakes move you back on the board because you have to slide down them. If you land exactly at the top of a snake, slide move the player all the way to the square at the bottom of the snake or chute. (even if you roll a double).
7.  Land exactly on the last square to win. The first person to reach the highest square on the board wins. But there's a twist! If you roll too high, your player "bounces" off the last square and moves back. You can only win by rolling the exact number needed to land on the last square. For example, if you are on square 98 and roll a five, move your game piece to 100 (two moves), then "bounce" back to 99, 98, 97 (three, four then five moves.)
8.  If the Player rolled a double and lands on the finish square “100” without any remaining moves then the Player wins the game and does not have to roll again.

Returns
Return Player n Wins!. Where n is winning player that has landed on square 100 without any remainding moves left.

Return Game over! if a player has won and another player tries to play.

Otherwise return Player n is on square x. Where n is the current player and x is the sqaure they are currently on.
*/

class SnakesLadders {

  public $players = [1,2];
  public $players_positions;
  public $player_turn = 0;
  public $snakes_ladders;
  public $size = 100;
  public $won = False;

  function __construct() {
    $this->snakes_ladders = $this->board($this->size);
    $this->players_positions = array_map(function () {return 0;}, $this->players);
  }

  public function game($players, $players_positions, $player_turn, $snakes_ladders, $board_size, $roll, $again = False){

    $player = $this->players[$player_turn];
    $new_pos = $players_positions[$player_turn] + $roll;
    if (array_key_exists($new_pos, $snakes_ladders))
        $new_pos = $snakes_ladders[$new_pos];
    if ($new_pos == $board_size){
        $this->won = True;
        return "Player " . $player . " Wins!";
    }
    if ($new_pos < $board_size) {
      $this->players_positions[$player_turn] = $new_pos;
      $this->player_turn = $this->next_player($this->player_turn, $again);
      return "Player " . $player . " is on square " . $new_pos;
    } else{
        $bounce = $board_size - ($new_pos - $board_size);
        $new_pos = $players_positions[$player_turn] = $bounce;
        if (array_key_exists($new_pos, $snakes_ladders))
        $new_pos = $snakes_ladders[$new_pos];
        $this->players_positions[$player_turn] = $new_pos;
        $this->player_turn = $this->next_player($this->player_turn, $again);
        return "Player " . $player . " is on square " . $new_pos;
    }
  }

  public function play($die1, $die2) {
    if($this->won == True){
        Return "Game over!";
    }

    $roll = $die1 + $die2;
    $again = $die1 === $die2 ? True : False;

    $game = $this->game($this->players, $this->players_positions, $this->player_turn, $this->snakes_ladders, $this->size, $roll, $again);
    return $game;
  }

  private function throw_dice()
  {
      return rand(1, 6);
  }
  private function random_cell_value($board_size)
  {
      return rand(1, $board_size);
  }

  function next_player($player_turn, $again = False)
  {
    if($again){
        return $player_turn;
    }else{
      if($player_turn == 1){
        return 0;
      }else{
        return 1;
      }
    }
  }

  private function board($board_size)
  {
    $snakes_ladders = [];

    //ladders
    $snakes_ladders[2] = 38;
    $snakes_ladders[7] = 14;
    $snakes_ladders[8] = 31;
    $snakes_ladders[15] = 26;
    $snakes_ladders[21] = 42;
    $snakes_ladders[28] = 84;
    $snakes_ladders[36] = 44;
    $snakes_ladders[51] = 67;
    $snakes_ladders[71] = 91;
    $snakes_ladders[78] = 98;
    $snakes_ladders[87] = 94;

    //snakers
    $snakes_ladders[16] = 6;
    $snakes_ladders[49] = 11;
    $snakes_ladders[46] = 25;
    $snakes_ladders[62] = 19;
    $snakes_ladders[74] = 53;
    $snakes_ladders[64] = 60;
    $snakes_ladders[89] = 68;
    $snakes_ladders[95] = 75;
    $snakes_ladders[99] = 80;
    $snakes_ladders[92] = 88;

    return $snakes_ladders;

  }

}
?>