function mark_horiz(row, from, to) {
  if (from < to) {
    for (col=from; col <= to; col++) {
      vents[row][col]++
    }
  } else {
    for (col=to; col <= from; col++) {
      vents[row][col]++
    }
  }
}

function mark_vert(col, from, to) {
  if (from < to) {
    for (row=from; row <= to; row++) {
      vents[row][col]++
    }
  } else {
    for (row=to; row <= from; row++) {
      vents[row][col]++
    }
  }
}

function mark_diag(x1, y1, x2, y2) {
  col = x1
  if (x1 < x2) {
    delta = 1
  } else {
    delta = -1
  }
  if (y1 < y2) {
    for (row = y1; row <= y2; row++) {
      vents[row][col]++
      col += delta
    }
  } else {
      for (row = y1; row >= y2; row--) {
      vents[row][col]++
      col += delta
    }
  }
}

BEGIN {
  FS="[->, ]"
}

{
  if ($1 == $6) {
    mark_vert($1, $2, $7)
  } else if ($2 == $7) {
    mark_horiz($2, $1, $6)
  } else {
    mark_diag($1, $2, $6, $7)
  }
}

END {
  count = 0
  for (row in vents) {
    for (col in vents[row]) {
      if (vents[row][col] > 1) {
        count++
      }
    }
  }
  print (count)
}
