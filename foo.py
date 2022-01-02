#  class Number:
#      def __init__(self, left, right):
#          self.left = left
#          self.right = right
#
#      def __str__(self):
#          return f"[{self.left},{self.right}]"
#
#      def __add__(self, other):
#          return Number(self, other)
#
#      def reduce(self):
#          try:
#              int(self.left)
#          except ValueError:
#              self.left.reduce()
#
#      @classmethod
#      def parse(cls, line):
#          stack = []
#          for c in line:
#              if c == '[':
#                  stack.append(cls(None, None))
#              elif c == ']':
#                  number = stack.pop()
#                  if not stack:
#                      return number
#
#                  if stack[-1].left is None:
#                      stack[-1].left = number
#                  else:
#                      assert(stack[-1].right is None)
#                      stack[-1].right = number
#              elif c == ',':
#                  pass
#              else:
#                  try:
#                      d = int(c)
#                      if stack[-1].left is None:
#                          stack[-1].left = d
#                      else:
#                          assert(stack[-1].right is None)
#                          stack[-1].right = d
#                  except ValueError:
#                      pass
#
#
#  with open('data/sample.txt') as f:
#      for line in f:
#          number = Number.parse(line)
#          print("----")
#          print(line.strip())
#          print(f"{number}")
#  #      numbers = [Number.parse(line.strip()) for line in f.readlines()]
#  #
#  #  number = numbers[0]
#  #
#  #  number = Number.parse("[2,[3,4]]")
#  #  print(number)
#
#  n1 = Number(1, 2)
#  n2 = Number(3, 4)
#  print(n1 + n2)

def explode_line(line):
    i = 0
    nesting = 0
    left = None
    right = None
    while nesting <= 4:
        if i >= len(line):
            return line, False

        if line[i].isdigit():
            left = i
        nesting += (line[i] == '[')
        i += 1

    j = 0
    while line[j] != ',':
        j += 1

    k = 0
    while line[k] != ']':
        k += 1

    for m in range(k+1, len(line)):
        if line[m].isdigit():
            right = m
            break

    a, b = (int(x) for x in line[i:k].split(','))
    if left is not None:
        x = int(line[left])
        x += a
        line = line[:left] + str(x) + line[left+1:]
    if right is not None:
        x = int(line[right])
        x += b
        line = line[:right] + str(x) + line[right+1:]

    return line[:i-1] + "0" + line[k+1:], True


def split_line(line):
    i = 0
    j = 0
    while i < len(line)-1:
        if i >= len(line)-1:
            return line, False

        if not line[i].isdigit():
            i += 1
            continue

        j = i
        while line[j].isdigit():
            j += 1

        if j > i+1:
            break

        i = j

    x = int(line[i:j])
    left = x / 2
    right = (x+1) / 2

    line = line[:i] + f"[{left},{right}]" + line[j:]

    return line, True


def reduce_line(line):
    exploded = False  # True
    split = True
    while exploded or split:
        #  line, exploded = explode_line(line)
        line, split = split_line(line)

    return line


with open('data/sample.txt') as f:
    lines = [line.strip() for line in f.readlines()]

line = lines[0]
print(line)

line = reduce_line(line)
print(line)

line = reduce_line(line)
print(line)
