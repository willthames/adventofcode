function binary_to_number(d) {
  result = 0
  for (i = 1; i <= length(d); i++) {
    v = substr(d, i, 1)
    if (v == "0") {
      result *= 2
    } else {
      result += result + 1
    }
  }
  return result
}

function copy_arr(from, to) {
  for (k in to) {
    if (!(k in from)) {
      delete to[k]
    }
  }
}

function whittle(arr, idx, most,       ones, zeroes) {
  for (k in arr) {
    if (substr(arr[k], idx+1, 1) == "1") {
      ones[k] = k
    } else {
      zeroes[k] = k
    }
  }
  if (length(ones) >= length(zeroes)) {
    if (most) {
      copy_arr(ones, arr)
    } else {
      copy_arr(zeroes, arr)
    }
  } else {
    if (most) {
      copy_arr(zeroes, arr)
    } else {
      copy_arr(ones, arr)
    }
  }
}

{
  oxygen[$1] = $1
  scrubber[$1] = $1
}


END {
  for (i=0; i< length($1); i++) {
    whittle(oxygen, i, 1)
    if (length(oxygen) <= 1) {
      break
    }
  }
  for (i=0; i< length($1); i++) {
    whittle(scrubber, i, 0)
    if (length(scrubber) <= 1) {
      break
    }
  }
  for (i in oxygen) {
    printf("oxygen %s ", oxygen[i])
  }
  for (i in scrubber) {
    printf("scrubber %s ", scrubber[i])
  }
  for (i in oxygen) {
    for (j in scrubber) {
      printf("result %d\n", binary_to_number(oxygen[i]) * binary_to_number(scrubber[j]))
    }
  }
}
