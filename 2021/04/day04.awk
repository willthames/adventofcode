function check_horiz(board, input) {
  for (row = 0; row<5; row++) {
    result = 1
    for (col = 0; col<5; col++) {
      if (!(board[row][col] in input)) {
        result = 0
      }
    }
    if (result == 1) {
      return 1
    }
  }
  return 0
}

function check_vert(board, input) {
  for (col = 0; col<5; col++) {
    result = 1
    for (row = 0; row<5; row++) {
      if (!(board[row][col] in input)) {
        result = 0
      }
    }
    if (result == 1) {
      return 1
    }
  }
  return 0
}

function calculate_board_score(board, input) {
  total = 0
  for (row = 0; row<5; row++) {
    for (col = 0; col<5; col++) {
      if (!(board[row][col] in input)) {
        total += board[row][col]
      }
    }
  }
  return total
}

BEGIN {
  FS=","
}

NR == 1 {
  for (i = 0; i<NF; i++) {
    ordered[i] = $i
  }
  FS=" "
}

NR > 1 {
  for (col = 0; col < 5 ; col++) {
    boards[int((NR-3)/6)][(NR-3)%6][col] = $(col+1)
  }
}

END {
  for (i = 0; i < length(ordered); i++) {
    input[ordered[i]] = ordered[i]

    for (board in boards) {
      if (check_horiz(boards[board], input) || check_vert(boards[board], input)) {
        board_score = calculate_board_score(boards[board], input)
        print board_score, ordered[i], board_score * ordered[i]
        exit
      }
    }
  }
}
