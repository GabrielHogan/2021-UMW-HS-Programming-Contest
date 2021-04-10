points = int(input())
periods = int(input())

# Print The Top
for top in range(points - 1, periods, 2):
    print((' ' * ((periods - top - 1) // 2)) + ('.' * (top + 1)))

# Print The Bottom
for bottom in range(periods - 2, points - 1, -2):
    print((' ' * ((periods - bottom) // 2)) + ('.' * bottom))