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

function max_key(arr) {
  max = 0
  if (!isarray(arr)) {
    return 0
  }
  for (key in arr) {
    if (key > max) {
      max = key
    }
  }
  return max
}

BEGIN {
  FS="[->, ]"
}

{
  if ($1 == $6) {
    mark_vert($1, $2, $7)
  } else if ($2 == $7) {
    mark_horiz($2, $1, $6)
  }
}

END {
  count = 0
  height = max_key(vents)
  for (row in vents) {
    for (col in vents[row]) {
    #  if (vents[row][col] >= 1) {
    #    printf("%02d ", vents[row][col])
    #  } else {
    #    printf(" . ", vents[row][col])
    #  }
      if (vents[row][col] > 1) {
        count++
      }
    }
  }
  print (count)
}
