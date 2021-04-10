import re

cases = int(input())
out = ''
count = 0

while count < cases:
    package = input()

    verdict = ''

    if package[0] != '[' or package[-1] != ']' or package.count('[') != package.count(']'):
        verdict = 'Incorrect'
    else:
        verdict = 'Correct'

    if count == cases - 1:
        out += verdict
    else:
        out += verdict + '\n'
    count += 1

print(out)