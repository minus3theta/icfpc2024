A = 1047197551

exp = 10
dig = -4
while True:
  exp = exp * 10
  if dig == 3:
    break
  dig = dig + 1

frac = 1
pow = A
denom = 1
psum = A
while True:
  if frac == 3:
    break
  pow = pow * A * A * -1
  denom = denom * (frac+1) * exp * (frac+2) * exp
  psum = psum * (frac+1) * exp * (frac+2) * exp + pow
  frac = frac + 1 + 1
print(psum // denom)
