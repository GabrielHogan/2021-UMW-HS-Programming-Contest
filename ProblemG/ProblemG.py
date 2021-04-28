width = int(input())
inputs = []
out = ''

while len(inputs) < width:
    inputs.append([int(i) for i in input().split()])


top = left = 0
bottom = len(inputs) - 1
right = len(inputs[0]) - 1

while True:
    if left > right:
        break

    for i in range(left, right + 1):
        out += str(inputs[top][i]) + ' '
    top = top + 1

    if top > bottom:
        break

    for i in range(top, bottom + 1):
        out += str(inputs[i][right]) + ' '
    right = right - 1

    if left > right:
        break

    for i in range(right, left - 1, -1):
        out += str(inputs[bottom][i]) + ' '
    bottom = bottom - 1

    if top > bottom:
        break

    for i in range(bottom, top - 1, -1):
        out += str(inputs[i][left]) + ' '
    left = left + 1

print(out.strip())
