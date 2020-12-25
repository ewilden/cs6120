
// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Example
ext_euclid(240n, 46n);

function quot(a: bigint, b: bigint): bigint {
  var q = 0n;
  for (; ;) {
    if (a >= 0n) {
      if (a < b) {
        return q;
      }
      if (a < 0n - b) {
        return q;
      }
    }
    q = q + 1n;
    a = a - b;
  }
}

function ext_euclid(a: bigint, b: bigint) {
  var r0 = a;
  var r1 = b;
  var s0 = 1n;
  var s1 = 0n;
  var t0 = 0n;
  var t1 = 1n;

  for (var r = 1n; r > 0n;) {
    var q = quot(r0, r1);
    r = r0 - q * r1;
    var s = s0 - q * s1;
    var t = t0 - q * t1;
    r0 = r1;
    s0 = s1;
    t0 = t1;
    r1 = r;
    s1 = s;
    t1 = t;
  }

  console.log(r0);
  console.log(s0);
  console.log(t0);
}
